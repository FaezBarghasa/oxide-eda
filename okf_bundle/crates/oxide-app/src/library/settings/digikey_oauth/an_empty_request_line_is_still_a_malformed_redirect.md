---
okf_version: "0.2"
type: Function
title: an_empty_request_line_is_still_a_malformed_redirect
description: An empty first line is what a failed read used to look like. It
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/an_empty_request_line_is_still_a_malformed_redirect
language: rust
---

# an_empty_request_line_is_still_a_malformed_redirect

An empty first line is what a failed read used to look like. It

## Signature

```rust
fn an_empty_request_line_is_still_a_malformed_redirect()
```

## Decorators

- `test`

## Docstring

An empty first line is what a failed read used to look like. It
still means "malformed redirect" when it really is one — a peer
that connected and sent nothing — so the two paths stay apart
only because the error is carried, not inferred from emptiness.
[test]

## Source
Lines 512–515 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [failed_reason](/crates/oxide-app/src/library/settings/digikey_oauth/failed_reason.md) |
| calls | [callback_params](/crates/oxide-app/src/library/settings/digikey_oauth/callback_params.md) |
