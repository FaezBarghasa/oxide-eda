---
okf_version: "0.2"
type: Function
title: fixture_response
resource: crates/oxide-library/tests/distributor_digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_digikey/fixture_response
language: rust
---

# fixture_response

## Signature

```rust
fn fixture_response(mpn: &str) -> serde_json::Value
```

## Source
Lines 21–36 in `crates/oxide-library/tests/distributor_digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_digikey](/crates/oxide-library/tests/distributor_digikey.md) |
| called_by | [lookup_by_mpn_with_inline_token](/crates/oxide-library/tests/distributor_digikey/lookup_by_mpn_with_inline_token.md) |
| called_by | [refresh_token_grant_calls_token_endpoint](/crates/oxide-library/tests/distributor_digikey/refresh_token_grant_calls_token_endpoint.md) |
