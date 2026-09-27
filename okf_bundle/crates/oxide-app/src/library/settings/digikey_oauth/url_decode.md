---
okf_version: "0.2"
type: Function
title: url_decode
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/url_decode
language: rust
---

# url_decode

## Signature

```rust
fn url_decode(s: &str) -> String
```

## Source
Lines 365–390 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [hex_digit](/crates/oxide-app/src/library/settings/digikey_oauth/hex_digit.md) |
| called_by | [parse_callback](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback.md) |
