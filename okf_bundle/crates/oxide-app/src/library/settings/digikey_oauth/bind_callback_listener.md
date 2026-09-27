---
okf_version: "0.2"
type: Function
title: bind_callback_listener
description: Bind a localhost port for the OAuth callback server. Returns the
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/bind_callback_listener
language: rust
---

# bind_callback_listener

Bind a localhost port for the OAuth callback server. Returns the

## Signature

```rust
fn bind_callback_listener() -> Result<(TcpListener, String), std::io::Error>
```

## Docstring

Bind a localhost port for the OAuth callback server. Returns the
listener (used to receive exactly one redirect) and the URL to
register with `DigiKeyAuth` as the redirect target.

We bind to `127.0.0.1` (loopback) only — never `0.0.0.0`. The
kernel chooses a random free port via `:0` so multiple parallel
flows don't collide.

## Source
Lines 111–116 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [run_blocking](/crates/oxide-app/src/library/settings/digikey_oauth/run_blocking.md) |
