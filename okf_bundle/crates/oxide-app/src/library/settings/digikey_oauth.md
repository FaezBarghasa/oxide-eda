---
okf_version: "0.2"
type: Module
title: digikey_oauth
description: DigiKey OAuth2 PKCE handshake — UI side.
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth
language: rust
---

# digikey_oauth

DigiKey OAuth2 PKCE handshake — UI side.

## Docstring

DigiKey OAuth2 PKCE handshake — UI side.

Flow:
- "Connect via OAuth" → `DigiKeyAuth::start_authorization` → open
the URL in the user's default browser via `webbrowser`.
- Spin up a one-shot HTTP server on `127.0.0.1:<random_port>` that
accepts the `?code=&state=` callback.
- Pass the returned `code` + the matching CSRF state back to
`DigiKeyAuth::exchange_code` (which internally persists the
refresh token via `KeyringStore`).

Why blocking, not async: the underlying `oauth2`/`reqwest` calls
that `oxide-library` exposes are blocking, and the iced runtime
happily spawns blocking work via `Task::perform` over `tokio`'s
`spawn_blocking`. Keeping the whole flow blocking inside one
function makes the borrow shape obvious and avoids needing a
parallel async branch in `oxide-library`.

Configuration:
- DigiKey client_id / client_secret are read from the environment
(`OXIDE_DIGIKEY_CLIENT_ID` / `OXIDE_DIGIKEY_CLIENT_SECRET`).
Unit tests use a wiremock server, so the constants are never
committed to source.

Cancellation:
- The caller holds a `CancelHandle` that, when dropped or via
`cancel()`, asks the listener to stop blocking on the next
`recv_timeout`. The handler observes cancellation and returns
[`Outcome::Cancelled`].

## Relationships

| Type | Target |
|------|--------|
| related | [Outcome](/crates/oxide-app/src/library/settings/digikey_oauth/Outcome.md) |
| related | [CancelHandle](/crates/oxide-app/src/library/settings/digikey_oauth/CancelHandle.md) |
| related | [new](/crates/oxide-app/src/library/settings/digikey_oauth/new.md) |
| related | [from_flag](/crates/oxide-app/src/library/settings/digikey_oauth/from_flag.md) |
| related | [cancel](/crates/oxide-app/src/library/settings/digikey_oauth/cancel.md) |
| related | [is_cancelled](/crates/oxide-app/src/library/settings/digikey_oauth/is_cancelled.md) |
| related | [new](/crates/oxide-app/src/library/settings/digikey_oauth/new.md) |
| related | [from_flag](/crates/oxide-app/src/library/settings/digikey_oauth/from_flag.md) |
| related | [cancel](/crates/oxide-app/src/library/settings/digikey_oauth/cancel.md) |
| related | [is_cancelled](/crates/oxide-app/src/library/settings/digikey_oauth/is_cancelled.md) |
| related | [default](/crates/oxide-app/src/library/settings/digikey_oauth/default.md) |
| related | [default](/crates/oxide-app/src/library/settings/digikey_oauth/default.md) |
| related | [bind_callback_listener](/crates/oxide-app/src/library/settings/digikey_oauth/bind_callback_listener.md) |
| related | [run_blocking](/crates/oxide-app/src/library/settings/digikey_oauth/run_blocking.md) |
| related | [failure_from](/crates/oxide-app/src/library/settings/digikey_oauth/failure_from.md) |
| related | [read_first_line](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line.md) |
| related | [callback_params](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params.md) |
| related | [callback_query_keys](/crates/oxide-app/src/library/settings/digikey_oauth/callback_query_keys.md) |
| related | [parse_callback](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback.md) |
| related | [url_decode](/crates/oxide-app/src/library/settings/digikey_oauth/url_decode.md) |
| related | [hex_digit](/crates/oxide-app/src/library/settings/digikey_oauth/hex_digit.md) |
| related | [read_env_credentials](/crates/oxide-app/src/library/settings/digikey_oauth/read_env_credentials.md) |
| related | [parse_callback_extracts_code_and_state](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback_extracts_code_and_state.md) |
| related | [parse_callback_handles_url_encoding](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback_handles_url_encoding.md) |
| related | [parse_callback_returns_none_on_missing_query](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback_returns_none_on_missing_query.md) |
| related | [FailingReader](/crates/oxide-app/src/library/settings/digikey_oauth/FailingReader.md) |
| related | [read](/crates/oxide-app/src/library/settings/digikey_oauth/read.md) |
| related | [read](/crates/oxide-app/src/library/settings/digikey_oauth/read.md) |
| related | [failed_reason](/crates/oxide-app/src/library/settings/digikey_oauth/failed_reason.md) |
| related | [read_first_line_returns_the_request_line](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_returns_the_request_line.md) |
| related | [read_first_line_propagates_a_socket_read_error](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_propagates_a_socket_read_error.md) |
| related | [callback_params_returns_code_and_state_on_a_good_redirect](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params_returns_code_and_state_on_a_good_redirect.md) |
| related | [socket_read_error_and_malformed_redirect_report_different_reasons](/crates/oxide-app/src/library/settings/digikey_oauth/socket_read_error_and_malformed_redirect_report_different_reasons.md) |
| related | [an_empty_request_line_is_still_a_malformed_redirect](/crates/oxide-app/src/library/settings/digikey_oauth/an_empty_request_line_is_still_a_malformed_redirect.md) |
| related | [callback_query_keys_logs_names_without_values](/crates/oxide-app/src/library/settings/digikey_oauth/callback_query_keys_logs_names_without_values.md) |
| related | [missing_client_id_fails_clearly](/crates/oxide-app/src/library/settings/digikey_oauth/missing_client_id_fails_clearly.md) |
| related | [cancel_handle_observable_across_clones](/crates/oxide-app/src/library/settings/digikey_oauth/cancel_handle_observable_across_clones.md) |
