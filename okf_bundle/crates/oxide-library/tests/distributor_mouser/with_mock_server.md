---
okf_version: "0.2"
type: Function
title: with_mock_server
resource: crates/oxide-library/tests/distributor_mouser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_mouser/with_mock_server
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
Lines 30–50 in `crates/oxide-library/tests/distributor_mouser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_mouser](/crates/oxide-library/tests/distributor_mouser.md) |
| called_by | [empty_search_results_returns_empty_vec](/crates/oxide-library/tests/distributor_mouser/empty_search_results_returns_empty_vec.md) |
| called_by | [http_401_surfaces_auth_error](/crates/oxide-library/tests/distributor_mouser/http_401_surfaces_auth_error.md) |
| called_by | [lookup_by_mpn_passes_apikey_header](/crates/oxide-library/tests/distributor_mouser/lookup_by_mpn_passes_apikey_header.md) |
