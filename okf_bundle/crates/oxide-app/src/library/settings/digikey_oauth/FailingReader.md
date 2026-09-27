---
okf_version: "0.2"
type: Class
title: FailingReader
description: "A socket that refuses every read, standing in for a connection"
resource: crates/oxide-app/src/library/settings/digikey_oauth.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/digikey_oauth/FailingReader
language: rust
---

# FailingReader

A socket that refuses every read, standing in for a connection

## Signature

```rust
struct FailingReader
```

## Docstring

A socket that refuses every read, standing in for a connection
reset, a timeout, or a port scanner that opens and drops.

## Source
Lines 436–436 in `crates/oxide-app/src/library/settings/digikey_oauth.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [digikey_oauth](/crates/oxide-app/src/library/settings/digikey_oauth.md) |
| called_by | [read_first_line_propagates_a_socket_read_error](/crates/oxide-app/src/library/settings/digikey_oauth/read_first_line_propagates_a_socket_read_error.md) |
