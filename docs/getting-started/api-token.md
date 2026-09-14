---
title: API token
description: imgen authenticates with a Replicate API token from the REPLICATE_API_TOKEN environment variable.
order: 2
---

## Create a token

Sign in at [replicate.com](https://replicate.com/) and create a token under
[Account → API tokens](https://replicate.com/account/api-tokens). Tokens start with `r8_`.

Replicate bills every prediction to the account that owns the token. Check the price on the
model page before large batches.

## Set it

imgen reads the token only from the environment. There is no flag and no config file.

```sh
export REPLICATE_API_TOKEN="r8_your_token_here"
```

Add the line to your shell profile to keep it across sessions, or use a per-project tool such as
direnv. imgen does not load `.env` files itself.

## When it is missing

The check runs before anything else, so no request is sent:

```text
$ imgen "A sunset over the ocean"
Error: Environment variable REPLICATE_API_TOKEN is not set

Caused by:
    environment variable not found
```

The exit code is 1.

## What the token is used for

Two kinds of requests carry it: the model lookup that reads the input schema
([Automatic schema detection](../../guides/schema-detection/)) and the prediction itself, including
polling for the result. An invalid token therefore fails at the model lookup, before any image is
generated.

Continue with the [Quickstart](../quickstart/).
