---
okf_version: "0.2"
type: Module
title: distributor_lcsc
description: LCSC adapter integration tests using wiremock.
resource: crates/oxide-library/tests/distributor_lcsc.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_lcsc
language: rust
---

# distributor_lcsc

LCSC adapter integration tests using wiremock.

## Docstring

LCSC adapter integration tests using wiremock.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_response](/crates/oxide-library/tests/distributor_lcsc/fixture_response.md) |
| related | [with_mock_server](/crates/oxide-library/tests/distributor_lcsc/with_mock_server.md) |
| related | [lookup_by_mpn_parses_wiremock_fixture](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_parses_wiremock_fixture.md) |
| related | [lookup_by_mpn_returns_empty_on_empty_product_list](/crates/oxide-library/tests/distributor_lcsc/lookup_by_mpn_returns_empty_on_empty_product_list.md) |
| related | [http_429_surfaces_rate_limited_with_retry_after](/crates/oxide-library/tests/distributor_lcsc/http_429_surfaces_rate_limited_with_retry_after.md) |
| related | [cache_hit_short_circuits_network](/crates/oxide-library/tests/distributor_lcsc/cache_hit_short_circuits_network.md) |
| related | [live_lookup_smoke](/crates/oxide-library/tests/distributor_lcsc/live_lookup_smoke.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
| related | [wiremock](/_dependencies/cargo/wiremock.md) |
