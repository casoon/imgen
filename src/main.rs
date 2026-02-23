use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const REPLICATE_API_BASE: &str = "https://api.replicate.com/v1";
const DEFAULT_MODEL: &str = "black-forest-labs/flux-1.1-pro";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const TIMEOUT: Duration = Duration::from_secs(120);

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

/// Generate images via the Replicate API.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Text prompt for image generation (ignored when --prompt-file is used).
    prompt: Option<String>,

    /// JSON file containing an array of jobs.
    #[arg(long, value_name = "FILE")]
    prompt_file: Option<PathBuf>,

    /// Replicate model in "owner/name" format.
    #[arg(long, default_value = DEFAULT_MODEL)]
    model: String,

    /// Output width in pixels.
    #[arg(long, default_value_t = 1024)]
    width: u32,

    /// Output height in pixels.
    #[arg(long, default_value_t = 768)]
    height: u32,

    /// Output file path (only used for single-prompt mode).
    #[arg(long, default_value = "output.png")]
    out: PathBuf,

    /// Convert output to WebP with given quality (0-100). Omit value for default 80.
    #[arg(long, default_missing_value = "80", num_args = 0..=1)]
    webp: Option<f32>,
}

// ---------------------------------------------------------------------------
// Prompt-file schema
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct PromptJob {
    prompt: String,
    out: PathBuf,
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    webp: Option<f32>,
}

#[derive(Debug, Default, Deserialize)]
struct JobDefaults {
    #[serde(default)]
    width: Option<u32>,
    #[serde(default)]
    height: Option<u32>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    webp: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct PromptFile {
    #[serde(default)]
    defaults: JobDefaults,
    jobs: Vec<PromptJob>,
}

// ---------------------------------------------------------------------------
// Replicate API types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct PredictionRequest {
    version: String,
    input: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct PredictionResponse {
    id: String,
    status: String,
    output: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
    urls: Option<PredictionUrls>,
}

#[derive(Debug, Deserialize)]
struct PredictionUrls {
    get: String,
}

#[derive(Debug, Deserialize)]
struct ModelResponse {
    latest_version: Option<ModelVersion>,
}

#[derive(Debug, Deserialize)]
struct ModelVersion {
    id: String,
    openapi_schema: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Model resolution
// ---------------------------------------------------------------------------

struct ResolvedModel {
    version: String,
    /// Allowed aspect_ratio values from the model schema, if the model uses aspect_ratio.
    aspect_ratios: Option<Vec<String>>,
}

async fn resolve_model(
    client: &Client,
    token: &str,
    model: &str,
) -> Result<ResolvedModel> {
    let url = format!("{REPLICATE_API_BASE}/models/{model}");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .with_context(|| format!("Failed to fetch model info for {model}"))?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        bail!("Failed to resolve model {model}: {status} {text}");
    }

    let model_resp: ModelResponse = resp
        .json()
        .await
        .context("Failed to parse model response")?;

    let ver = model_resp
        .latest_version
        .context(format!("Model {model} has no published version"))?;

    let aspect_ratios = ver
        .openapi_schema
        .as_ref()
        .and_then(|s| s.pointer("/components/schemas/Input/properties/aspect_ratio"))
        .and_then(|ar| {
            ar.get("enum")
                .and_then(|e| e.as_array())
                .map(|vals| {
                    vals.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
        });

    Ok(ResolvedModel {
        version: ver.id,
        aspect_ratios,
    })
}

/// Pick the aspect_ratio string closest to width/height from the allowed values.
/// Ratios are expected as "W:H" strings. Returns "custom" if available, otherwise
/// the closest match by comparing w/h floats.
fn best_aspect_ratio(width: u32, height: u32, allowed: &[String]) -> String {
    // If "custom" is available, use it so we can pass exact width/height.
    if allowed.iter().any(|r| r == "custom") {
        return "custom".into();
    }

    let target = width as f64 / height as f64;

    allowed
        .iter()
        .filter_map(|r| {
            let parts: Vec<&str> = r.split(':').collect();
            if parts.len() != 2 {
                return None;
            }
            let w: f64 = parts[0].parse().ok()?;
            let h: f64 = parts[1].parse().ok()?;
            Some((r.clone(), (w / h - target).abs()))
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(r, _)| r)
        .unwrap_or_else(|| "1:1".into())
}

fn build_input(
    prompt: &str,
    width: u32,
    height: u32,
    aspect_ratios: &Option<Vec<String>>,
) -> serde_json::Value {
    match aspect_ratios {
        Some(ratios) => {
            let ratio = best_aspect_ratio(width, height, ratios);
            if ratio == "custom" {
                serde_json::json!({
                    "prompt": prompt,
                    "aspect_ratio": "custom",
                    "width": width,
                    "height": height,
                })
            } else {
                // Model uses fixed ratios; don't send width/height.
                serde_json::json!({
                    "prompt": prompt,
                    "aspect_ratio": ratio,
                })
            }
        }
        None => {
            // No aspect_ratio param — send width/height directly.
            serde_json::json!({
                "prompt": prompt,
                "width": width,
                "height": height,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Core logic
// ---------------------------------------------------------------------------

async fn create_prediction(
    client: &Client,
    token: &str,
    version: &str,
    input: serde_json::Value,
) -> Result<PredictionResponse> {
    let body = PredictionRequest {
        version: version.to_string(),
        input,
    };

    let resp = client
        .post(format!("{REPLICATE_API_BASE}/predictions"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .context("Failed to send prediction request")?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        bail!("Replicate API returned {status}: {text}");
    }

    resp.json::<PredictionResponse>()
        .await
        .context("Failed to parse prediction response")
}

async fn poll_prediction(
    client: &Client,
    token: &str,
    prediction: &PredictionResponse,
) -> Result<String> {
    let poll_url = prediction
        .urls
        .as_ref()
        .map(|u| u.get.clone())
        .unwrap_or_else(|| {
            format!("{REPLICATE_API_BASE}/predictions/{}", prediction.id)
        });

    let start = Instant::now();

    loop {
        if start.elapsed() > TIMEOUT {
            bail!(
                "Timeout: prediction {} did not finish within {} seconds",
                prediction.id,
                TIMEOUT.as_secs()
            );
        }

        tokio::time::sleep(POLL_INTERVAL).await;

        let resp = client
            .get(&poll_url)
            .bearer_auth(token)
            .send()
            .await
            .context("Failed to poll prediction")?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            bail!("Replicate API returned {status} during polling: {text}");
        }

        let pred: PredictionResponse = resp
            .json()
            .await
            .context("Failed to parse poll response")?;

        match pred.status.as_str() {
            "succeeded" => {
                let image_url = extract_image_url(&pred)?;
                return Ok(image_url);
            }
            "failed" | "canceled" => {
                let err_detail = pred
                    .error
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "unknown error".into());
                bail!(
                    "Prediction {} {}: {}",
                    pred.id,
                    pred.status,
                    err_detail
                );
            }
            _ => {
                // starting / processing – keep polling
            }
        }
    }
}

fn extract_image_url(pred: &PredictionResponse) -> Result<String> {
    let output = pred
        .output
        .as_ref()
        .context("Prediction succeeded but output is missing")?;

    if let Some(url) = output.as_str() {
        return Ok(url.to_string());
    }
    if let Some(arr) = output.as_array() {
        if let Some(first) = arr.first().and_then(|v| v.as_str()) {
            return Ok(first.to_string());
        }
    }

    bail!("Unexpected output format: {output}");
}

async fn download_image(client: &Client, url: &str, path: &Path) -> Result<()> {
    let bytes = client
        .get(url)
        .send()
        .await
        .context("Failed to download image")?
        .bytes()
        .await
        .context("Failed to read image bytes")?;

    if let Some(parent) = path.parent() {
        if parent != std::path::Path::new("") {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
    }

    std::fs::write(path, &bytes)
        .with_context(|| format!("Failed to write image to {}", path.display()))?;

    Ok(())
}

fn convert_to_webp(png_path: &Path, quality: f32) -> Result<PathBuf> {
    let img = image::open(png_path)
        .with_context(|| format!("Failed to open {} for WebP conversion", png_path.display()))?;

    let encoder = webp::Encoder::from_image(&img)
        .map_err(|e| anyhow::anyhow!("WebP encoder error: {e}"))?;

    let webp_data = encoder.encode(quality);

    let webp_path = png_path.with_extension("webp");
    std::fs::write(&webp_path, &*webp_data)
        .with_context(|| format!("Failed to write {}", webp_path.display()))?;

    // Remove the original PNG after successful conversion.
    std::fs::remove_file(png_path)
        .with_context(|| format!("Failed to remove {}", png_path.display()))?;

    Ok(webp_path)
}

async fn generate_image(
    client: &Client,
    token: &str,
    prompt: &str,
    width: u32,
    height: u32,
    resolved: &ResolvedModel,
    out: &Path,
    webp_quality: Option<f32>,
) -> Result<()> {
    eprintln!("Creating prediction for: \"{prompt}\"");
    let input = build_input(prompt, width, height, &resolved.aspect_ratios);
    let pred = create_prediction(client, token, &resolved.version, input).await?;
    eprintln!("Prediction {} created (status: {})", pred.id, pred.status);

    let image_url = if pred.status == "succeeded" {
        extract_image_url(&pred)?
    } else {
        eprintln!("Polling for result…");
        poll_prediction(client, token, &pred).await?
    };

    eprintln!("Downloading image…");
    download_image(client, &image_url, out).await?;

    if let Some(quality) = webp_quality {
        eprintln!("Converting to WebP (quality {quality})…");
        let webp_path = convert_to_webp(out, quality)?;
        eprintln!("Saved to {}", webp_path.display());
    } else {
        eprintln!("Saved to {}", out.display());
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let token = std::env::var("REPLICATE_API_TOKEN")
        .context("Environment variable REPLICATE_API_TOKEN is not set")?;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("Failed to build HTTP client")?;

    let jobs = build_jobs(&cli)?;

    // Resolve all unique models upfront to avoid redundant API calls.
    let mut model_cache: HashMap<String, ResolvedModel> = HashMap::new();
    for job in &jobs {
        let model = job.model.as_deref().unwrap_or(&cli.model);
        if !model_cache.contains_key(model) {
            eprintln!("Resolving model {model}…");
            let resolved = resolve_model(&client, &token, model).await?;
            eprintln!(
                "  version: {}…  aspect_ratio: {}",
                resolved.version.get(..12).unwrap_or(&resolved.version),
                match &resolved.aspect_ratios {
                    Some(ratios) => ratios.join(", "),
                    None => "no".into(),
                }
            );
            model_cache.insert(model.to_string(), resolved);
        }
    }

    let mut had_error = false;

    for (i, job) in jobs.iter().enumerate() {
        if jobs.len() > 1 {
            eprintln!("\n[{}/{}]", i + 1, jobs.len());
        }

        let model = job.model.as_deref().unwrap_or(&cli.model);
        let resolved = &model_cache[model];

        if let Err(e) = generate_image(
            &client,
            &token,
            &job.prompt,
            job.width.unwrap_or(cli.width),
            job.height.unwrap_or(cli.height),
            resolved,
            &job.out,
            job.webp.or(cli.webp),
        )
        .await
        {
            eprintln!("Error: {e:#}");
            had_error = true;
        }
    }

    if had_error {
        bail!("One or more jobs failed");
    }

    Ok(())
}

fn build_jobs(cli: &Cli) -> Result<Vec<PromptJob>> {
    if let Some(ref path) = cli.prompt_file {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read prompt file {}", path.display()))?;

        let (defaults, mut jobs) = parse_prompt_file(&content)
            .with_context(|| format!("Failed to parse prompt file {}", path.display()))?;

        if jobs.is_empty() {
            bail!("Prompt file contains no jobs");
        }

        // Merge defaults into jobs: job values take priority over defaults.
        for job in &mut jobs {
            job.width = job.width.or(defaults.width);
            job.height = job.height.or(defaults.height);
            job.model = job.model.take().or_else(|| defaults.model.clone());
            job.webp = job.webp.or(defaults.webp);
        }

        Ok(jobs)
    } else {
        let prompt = cli
            .prompt
            .clone()
            .context("Either a prompt argument or --prompt-file is required")?;

        Ok(vec![PromptJob {
            prompt,
            out: cli.out.clone(),
            width: None,
            height: None,
            model: None,
            webp: None,
        }])
    }
}

/// Parse prompt file supporting two formats:
/// 1. Plain array: `[{ "prompt": ..., "out": ... }, ...]`
/// 2. Object with defaults: `{ "defaults": { ... }, "jobs": [...] }`
fn parse_prompt_file(content: &str) -> Result<(JobDefaults, Vec<PromptJob>)> {
    let value: serde_json::Value =
        serde_json::from_str(content).context("Invalid JSON")?;

    if value.is_array() {
        let jobs: Vec<PromptJob> = serde_json::from_value(value)
            .context("Failed to parse jobs array")?;
        Ok((JobDefaults::default(), jobs))
    } else if value.is_object() {
        let file: PromptFile = serde_json::from_value(value)
            .context("Failed to parse prompt file object (expected \"defaults\" + \"jobs\")")?;
        Ok((file.defaults, file.jobs))
    } else {
        bail!("Prompt file must be a JSON array or an object with \"jobs\"");
    }
}
