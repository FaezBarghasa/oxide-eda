---
okf_version: "0.2"
type: Function
title: callback_query_keys
description: "Names of the query parameters on a callback request line, joined for"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/callback_query_keys
language: rust
---

# callback_query_keys

Names of the query parameters on a callback request line, joined for

## Signature

```rust
fn callback_query_keys(req_line: &str) -> String
```

## Docstring

Names of the query parameters on a callback request line, joined for
the log record. Values are deliberately left out — one of them is the
authorization code, and the Messages panel is read out loud in bug
reports.

## Source
Lines 328–340 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [callback_query_keys_logs_names_without_values](/crates/oxide-app/src/library/settings/digikey_oauth/callback_query_keys_logs_names_without_values.md) |
