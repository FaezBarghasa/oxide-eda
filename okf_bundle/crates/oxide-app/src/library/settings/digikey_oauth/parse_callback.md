---
okf_version: "0.2"
type: Function
title: parse_callback
description: "Extract `code` and `state` query params from the first request"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/parse_callback
language: rust
---

# parse_callback

Extract `code` and `state` query params from the first request

## Signature

```rust
fn parse_callback(req_line: &str) -> Option<(String, String)>
```

## Docstring

Extract `code` and `state` query params from the first request
line `GET /callback?code=...&state=... HTTP/1.1`.

## Source
Lines 344–363 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [url_decode](/crates/oxide-app/src/library/settings/digikey_oauth/url_decode.md) |
| called_by | [callback_params](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params.md) |
| called_by | [parse_callback_extracts_code_and_state](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback_extracts_code_and_state.md) |
| called_by | [parse_callback_handles_url_encoding](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback_handles_url_encoding.md) |
