---
okf_version: "0.2"
type: Function
title: fixture_response
resource: crates/oxide-library/tests/distributor_jlcpcb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-library/tests/distributor_jlcpcb/fixture_response
language: rust
---

# fixture_response

## Signature

```rust
fn fixture_response(mpn: &str) -> serde_json::Value
```

## Source
Lines 13–29 in `crates/oxide-library/tests/distributor_jlcpcb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_jlcpcb](/crates/oxide-library/tests/distributor_jlcpcb.md) |
| called_by | [lookup_by_mpn_parses_wiremock_fixture](/crates/oxide-library/tests/distributor_jlcpcb/lookup_by_mpn_parses_wiremock_fixture.md) |
