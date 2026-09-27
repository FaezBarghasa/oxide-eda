---
okf_version: "0.2"
type: Function
title: from_symbol
description: Build a new container holding a single symbol — what the
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/from_symbol_1
language: rust
---

# from_symbol

Build a new container holding a single symbol — what the

## Signature

```rust
pub fn from_symbol(symbol: Symbol) -> Self
```

## Visibility

- `pub`

## Docstring

Build a new container holding a single symbol — what the
`Add New ▸ Symbol` flow seeds.

## Source
Lines 657–667 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| calls | [default_format](/crates/oxide-library/src/primitive/symbol/mod/default_format.md) |
