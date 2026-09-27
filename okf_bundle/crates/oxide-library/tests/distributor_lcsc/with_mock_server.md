---
okf_version: "0.2"
type: Function
title: with_mock_server
description: Spawn a multi-thread Tokio runtime in a background thread so wiremock
resource: crates/oxide-library/tests/distributor_lcsc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_lcsc/with_mock_server
language: rust
---

# with_mock_server

Spawn a multi-thread Tokio runtime in a background thread so wiremock

## Signature

```rust
fn with_mock_server(setup: S, test: T)
```

## Type Parameters

- `S`
- `T`

## Docstring

Spawn a multi-thread Tokio runtime in a background thread so wiremock
has somewhere to live; the main test thread runs the blocking
`LcscAdapter` calls. Returns once `test` completes.

## Source
Lines 34–56 in `crates/oxide-library/tests/distributor_lcsc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_lcsc](/crates/oxide-library/tests/distributor_lcsc.md) |
| called_by | [cache_hit_short_circuits_network](/crates/oxide-library/tests/distributor_lcsc/cache_hit_short_circuits_network.md) |
| called_by | [http_429_surfaces_rate_limited_with_retry_after](/crates/oxide-library/tests/distributor_lcsc/http_429_surfaces_rate_limited_with_retry_after.md) |
| called_by | [lookup_by_mpn_parses_wiremock_fixture](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_parses_wiremock_fixture.md) |
| called_by | [lookup_by_mpn_returns_empty_on_empty_product_list](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_returns_empty_on_empty_product_list.md) |
