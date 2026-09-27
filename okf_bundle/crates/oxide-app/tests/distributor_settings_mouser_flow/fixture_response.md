---
okf_version: "0.2"
type: Function
title: fixture_response
resource: crates/oxide-app/tests/distributor_settings_mouser_flow.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/distributor_settings_mouser_flow/fixture_response
language: rust
---

# fixture_response

## Signature

```rust
fn fixture_response(mpn: &str) -> serde_json::Value
```

## Source
Lines 33–48 in `crates/oxide-app/tests/distributor_settings_mouser_flow.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_settings_mouser_flow](/crates/oxide-app/tests/distributor_settings_mouser_flow.md) |
| called_by | [mouser_test_flow_uses_sentinel_mpn_and_apikey_header](/crates/oxide-app/tests/distributor_settings_mouser_flow/mouser_test_flow_uses_sentinel_mpn_and_apikey_header.md) |
