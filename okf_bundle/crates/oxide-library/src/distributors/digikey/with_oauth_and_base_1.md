---
okf_version: "0.2"
type: Function
title: with_oauth_and_base
description: "Test+production constructor: provide a `DigiKeyAuth` that points at"
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/with_oauth_and_base_1
language: rust
---

# with_oauth_and_base

Test+production constructor: provide a `DigiKeyAuth` that points at

## Signature

```rust
pub fn with_oauth_and_base(
        api_base: impl Into<String>,
        auth: DigiKeyAuth,
        cache: Option<DistributorCache>,
    ) -> Self
```

## Visibility

- `pub`

## Docstring

Test+production constructor: provide a `DigiKeyAuth` that points at
a wiremock token endpoint. Combined with [`Self::with_access_token`]
this lets us cover the full OAuth flow in tests.

## Source
Lines 321–333 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| calls | [build_http_client](/crates/oxide-library/src/distributors/digikey/build_http_client.md) |
