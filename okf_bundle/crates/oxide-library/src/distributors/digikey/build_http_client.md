---
okf_version: "0.2"
type: Function
title: build_http_client
resource: crates/oxide-library/src/distributors/digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:46:03Z"
concept_id: crates/oxide-library/src/distributors/digikey/build_http_client
language: rust
---

# build_http_client

## Signature

```rust
fn build_http_client() -> reqwest::blocking::Client
```

## Source
Lines 260–266 in `crates/oxide-library/src/distributors/digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey](/crates/oxide-library/src/distributors/digikey.md) |
| called_by | [new](/crates/oxide-library/src/distributors/digikey/new.md) |
| called_by | [with_access_token](/crates/oxide-library/src/distributors/digikey/with_access_token.md) |
| called_by | [with_oauth_and_base](/crates/oxide-library/src/distributors/digikey/with_oauth_and_base.md) |
