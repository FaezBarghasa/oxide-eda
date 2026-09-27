---
okf_version: "0.2"
type: Function
title: get_symbol_mut
description: Locate a symbol by UUID within this file (mutable).
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/get_symbol_mut_1
language: rust
---

# get_symbol_mut

Locate a symbol by UUID within this file (mutable).

## Signature

```rust
pub fn get_symbol_mut(&mut self, uuid: Uuid) -> Option<&mut Symbol>
```

## Visibility

- `pub`

## Docstring

Locate a symbol by UUID within this file (mutable).

## Source
Lines 675–677 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
