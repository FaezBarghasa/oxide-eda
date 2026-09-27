---
okf_version: "0.2"
type: Function
title: with_access_token
description: "Test constructor: override the API base + provide a fixed access"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/with_access_token
language: rust
---

# with_access_token

Test constructor: override the API base + provide a fixed access

## Signature

```rust
impl DigiKeyAdapter { pub fn with_access_token(
        api_base: impl Into<String>,
        access_token: impl Into<String>,
        cache: Option<DistributorCache>,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Test constructor: override the API base + provide a fixed access
token. Skips the OAuth refresh flow entirely so wiremock can focus
on the product-search request.

## Source
Lines 304–316 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [build_http_client](/crates/oxide-library/src/distributors/digikey/build_http_client.md) |
