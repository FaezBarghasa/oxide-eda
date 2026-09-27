---
okf_version: "0.2"
type: Function
title: run_blocking
description: Body of the synchronous handshake. Runs on a worker thread (caller
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/run_blocking
language: rust
---

# run_blocking

Body of the synchronous handshake. Runs on a worker thread (caller

## Signature

```rust
pub fn run_blocking(
    client_id: String,
    client_secret: String,
    auth_url_endpoint: String,
    token_url_endpoint: String,
    cancel: CancelHandle,
    open_browser: bool,
) -> Outcome
```

## Visibility

- `pub`

## Docstring

Body of the synchronous handshake. Runs on a worker thread (caller
wraps this in `Task::perform` over `tokio::task::spawn_blocking`).

`auth_url_endpoint` / `token_url_endpoint` let tests redirect at a
wiremock instance; production callers pass the DigiKey constants
from `oxide-library`.

The function is split so the `cargo test` path can drive it end-
to-end against wiremock without needing a real browser.

## Source
Lines 127–250 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [bind_callback_listener](/crates/oxide-app/src/library/settings/digikey_oauth/bind_callback_listener.md) |
| calls | [failure_from](/crates/oxide-app/src/library/settings/digikey_oauth/failure_from.md) |
| calls | [read_first_line](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line.md) |
| calls | [callback_params](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params.md) |
| called_by | [handle_library_settings_message](/crates/oxide-app/src/app/dispatch/library/settings/handle_library_settings_message.md) |
| called_by | [missing_client_id_fails_clearly](/crates/oxide-app/src/library/settings/digikey_oauth/missing_client_id_fails_clearly.md) |
