---
okf_version: "0.2"
type: Function
title: socket_read_error_and_malformed_redirect_report_different_reasons
description: "The row this test pins: a socket that could not be read and a"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/socket_read_error_and_malformed_redirect_report_different_reasons
language: rust
---

# socket_read_error_and_malformed_redirect_report_different_reasons

The row this test pins: a socket that could not be read and a

## Signature

```rust
fn socket_read_error_and_malformed_redirect_report_different_reasons()
```

## Decorators

- `test`

## Docstring

The row this test pins: a socket that could not be read and a
redirect that genuinely carries no `code`/`state` are two
different diagnoses and must not share one reason string. Before
the fix both produced "redirect missing code/state", sending a
user whose loopback connection was reset off to inspect the
redirect URI registered on their DigiKey app.
[test]

## Source
Lines 479–505 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [failed_reason](/crates/oxide-app/src/library/settings/digikey_oauth/failed_reason.md) |
| calls | [callback_params](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params.md) |
