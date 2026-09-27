---
okf_version: "0.2"
type: Module
title: distributor_settings_mouser_flow
description: Wiremock-backed validation for the Mouser test flow.
resource: crates/oxide-app/tests/distributor_settings_mouser_flow.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/distributor_settings_mouser_flow
language: rust
---

# distributor_settings_mouser_flow

Wiremock-backed validation for the Mouser test flow.

## Docstring

Wiremock-backed validation for the Mouser test flow.

The oxide-app handler runs `MouserAdapter::lookup_by_mpn(SENTINEL)`
against the Mouser API; on success it writes the API key to the OS
keyring. This test mirrors `oxide-library/tests/distributor_mouser.rs`
by hitting the same code path against a wiremock instance — proves
the app's choice of sentinel MPN + adapter wiring matches the
library-level integration shape.

We don't test the keyring writeback here because it depends on a
real OS keyring backend (Windows Credential Manager / Secret
Service); the writeback path lives behind one extra `if Ok` arm in
the dispatcher and is covered by the underlying
`KeyringStore::set_secret` tests in `oxide-library`.

## Relationships

| Type | Target |
|------|--------|
| related | [fixture_response](/crates/oxide-app/tests/distributor_settings_mouser_flow/fixture_response.md) |
| related | [with_mock_server](/crates/oxide-app/tests/distributor_settings_mouser_flow/with_mock_server.md) |
| related | [mouser_test_flow_uses_sentinel_mpn_and_apikey_header](/crates/oxide-app/tests/distributor_settings_mouser_flow/mouser_test_flow_uses_sentinel_mpn_and_apikey_header.md) |
| related | [mouser_test_flow_surfaces_auth_error_on_401](/crates/oxide-app/tests/distributor_settings_mouser_flow/mouser_test_flow_surfaces_auth_error_on_401.md) |
| related | [mouser_test_flow_handles_empty_search_results](/crates/oxide-app/tests/distributor_settings_mouser_flow/mouser_test_flow_handles_empty_search_results.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
| related | [wiremock](/_dependencies/cargo/wiremock.md) |
