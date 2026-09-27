---
okf_version: "0.2"
type: Function
title: callback_params
description: "Turn what came off the callback socket into the `(code, state)` pair,"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/callback_params
language: rust
---

# callback_params

Turn what came off the callback socket into the `(code, state)` pair,

## Signature

```rust
fn callback_params(req_line: Result<String, std::io::Error>) -> Result<(String, String), Outcome>
```

## Docstring

Turn what came off the callback socket into the `(code, state)` pair,
or into the [`Outcome::Failed`] that names which of the two very
different failures happened: the socket could not be read, or a
redirect did arrive and carries no `code`/`state`.

Both arms report to the Messages panel at `error!` — the connect
attempt ended with no account connected either way, and the status
line in the settings panel is overwritten by the next attempt.

## Source
Lines 285–322 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [parse_callback](/crates/oxide-app/src/library/settings/digikey_oauth/parse_callback.md) |
| called_by | [an_empty_request_line_is_still_a_malformed_redirect](/crates/oxide-app/src/library/settings/digikey_oauth/an_empty_request_line_is_still_a_malformed_redirect.md) |
| called_by | [callback_params_returns_code_and_state_on_a_good_redirect](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params_returns_code_and_state_on_a_good_redirect.md) |
| called_by | [run_blocking](/crates/oxide-app/src/library/settings/digikey_oauth/run_blocking.md) |
| called_by | [socket_read_error_and_malformed_redirect_report_different_reasons](/crates/oxide-app/src/library/settings/digikey_oauth/socket_read_error_and_malformed_redirect_report_different_reasons.md) |
