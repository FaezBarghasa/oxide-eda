---
okf_version: "0.2"
type: Function
title: peek_byte
description: Look at the byte at the current cursor without consuming it.
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/peek_byte_1
language: rust
---

# peek_byte

Look at the byte at the current cursor without consuming it.

## Signature

```rust
fn peek_byte(&self) -> Option<u8>
```

## Docstring

Look at the byte at the current cursor without consuming it.

## Source
Lines 122–124 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
