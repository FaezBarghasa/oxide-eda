---
okf_version: "0.2"
type: Module
title: distributor_digikey
description: DigiKey adapter integration tests using wiremock.
resource: crates/oxide-library/tests/distributor_digikey.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_digikey
language: rust
---

# distributor_digikey

DigiKey adapter integration tests using wiremock.

## Docstring

DigiKey adapter integration tests using wiremock.

Two flavours:
1. Inline access token + mocked product-search endpoint — covers the
happy path of the adapter without the OAuth dance.
2. Mocked OAuth2 token endpoint — exercises the refresh-token-grant
path that production runs on every API call. Refresh token comes
from the OS keyring; we plant it before the test runs.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_response](/crates/oxide-library/tests/distributor_digikey/fixture_response.md) |
| related | [with_mock_server](/crates/oxide-library/tests/distributor_digikey/with_mock_server.md) |
| related | [lookup_by_mpn_with_inline_token](/crates/oxide-library/tests/distributor_digikey/lookup_by_mpn_with_inline_token.md) |
| related | [http_401_surfaces_auth_error](/crates/oxide-library/tests/distributor_digikey/http_401_surfaces_auth_error.md) |
| related | [refresh_token_grant_calls_token_endpoint](/crates/oxide-library/tests/distributor_digikey/refresh_token_grant_calls_token_endpoint.md) |
| related | [no_refresh_token_yields_auth_error](/crates/oxide-library/tests/distributor_digikey/no_refresh_token_yields_auth_error.md) |
| related | [live_lookup_smoke](/crates/oxide-library/tests/distributor_digikey/live_lookup_smoke.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
| related | [wiremock](/_dependencies/cargo/wiremock.md) |
