---
okf_version: "0.2"
type: Function
title: with_endpoints
description: "Test constructor: override auth + token URLs (e.g. wiremock)."
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/with_endpoints_1
language: rust
---

# with_endpoints

Test constructor: override auth + token URLs (e.g. wiremock).

## Signature

```rust
pub fn with_endpoints(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
        auth_url: &str,
        token_url: &str,
    ) -> Result<Self, DigiKeyAuthError>
```

## Visibility

- `pub`

## Docstring

Test constructor: override auth + token URLs (e.g. wiremock).

## Source
Lines 112–143 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
