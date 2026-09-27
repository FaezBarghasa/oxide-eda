---
okf_version: "0.2"
type: Function
title: hex_short
description: Render the first 8 bytes of a SHA-256 hash as hex for trace logs.
resource: crates/oxide-app/src/app/handlers/library_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/library_place/hex_short
language: rust
---

# hex_short

Render the first 8 bytes of a SHA-256 hash as hex for trace logs.

## Signature

```rust
fn hex_short(hash: &[u8; 32]) -> String
```

## Docstring

Render the first 8 bytes of a SHA-256 hash as hex for trace logs.

## Source
Lines 120–126 in `crates/oxide-app/src/app/handlers/library_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_place](/crates/oxide-app/src/app/handlers/library_place.md) |
