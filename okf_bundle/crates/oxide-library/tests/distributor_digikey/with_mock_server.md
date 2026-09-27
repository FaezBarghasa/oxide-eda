---
okf_version: "0.2"
type: Function
title: with_mock_server
resource: crates/oxide-library/tests/distributor_digikey.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_digikey/with_mock_server
language: rust
---

# with_mock_server

## Signature

```rust
fn with_mock_server(setup: S, test: T)
```

## Type Parameters

- `S`
- `T`

## Source
Lines 38–58 in `crates/oxide-library/tests/distributor_digikey.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_digikey](/crates/oxide-library/tests/distributor_digikey.md) |
| called_by | [http_401_surfaces_auth_error](/crates/oxide-library/tests/distributor_digikey/http_401_surfaces_auth_error.md) |
| called_by | [lookup_by_mpn_with_inline_token](/crates/oxide-library/tests/distributor_digikey/lookup_by_mpn_with_inline_token.md) |
| called_by | [refresh_token_grant_calls_token_endpoint](/crates/oxide-library/tests/distributor_digikey/refresh_token_grant_calls_token_endpoint.md) |
