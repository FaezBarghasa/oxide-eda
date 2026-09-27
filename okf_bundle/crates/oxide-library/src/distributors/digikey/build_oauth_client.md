---
okf_version: "0.2"
type: Function
title: build_oauth_client
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/build_oauth_client
language: rust
---

# build_oauth_client

## Signature

```rust
fn build_oauth_client() -> oauth2::reqwest::blocking::Client
```

## Source
Lines 252–258 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| called_by | [access_token](/crates/oxide-library/src/distributors/digikey/access_token.md) |
| called_by | [exchange_code](/crates/oxide-library/src/distributors/digikey/exchange_code.md) |
