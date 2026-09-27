---
okf_version: "0.2"
type: Function
title: read_first_line_propagates_a_socket_read_error
description: "[test]"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_propagates_a_socket_read_error
language: rust
---

# read_first_line_propagates_a_socket_read_error

[test]

## Signature

```rust
fn read_first_line_propagates_a_socket_read_error()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 459–463 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| calls | [FailingReader](/crates/oxide-app/src/library/settings/digikey_oauth/FailingReader.md) |
| calls | [read_first_line](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line.md) |
