---
okf_version: "0.2"
type: Function
title: read_first_line
description: Read the first line of the HTTP request — only the line we need
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/read_first_line
language: rust
---

# read_first_line

Read the first line of the HTTP request — only the line we need

## Signature

```rust
fn read_first_line(stream: &mut R) -> Result<String, std::io::Error>
```

## Type Parameters

- `R: std::io::Read`

## Docstring

Read the first line of the HTTP request — only the line we need
to extract `?code=&state=` from. We intentionally avoid pulling in
the full `tiny_http` server here because it's overkill for one
request and adds a fork of the request lifecycle that doesn't
blend with the polling/cancel pattern. (We still link to it via
`Cargo.toml` for symmetry with the WS specs in case the flow
grows; the polled-listener version above is what runs.)

The read error is returned rather than folded into an empty string:
an empty line parses as "the redirect carried no code/state", which
is a completely different diagnosis from "the socket could not be
read at all" and sends the user to inspect the wrong thing.

## Source
Lines 270–275 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [read_first_line_propagates_a_socket_read_error](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_propagates_a_socket_read_error.md) |
| called_by | [read_first_line_returns_the_request_line](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_returns_the_request_line.md) |
| called_by | [run_blocking](/crates/oxide-app/src/library/settings/digikey_oauth/run_blocking.md) |
