---
okf_version: "0.2"
type: Function
title: upsert
description: "Replace `symbol` in the container — matches by `symbol.uuid`."
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/upsert
language: rust
---

# upsert

Replace `symbol` in the container — matches by `symbol.uuid`.

## Signature

```rust
impl SymbolFile { pub fn upsert(&mut self, symbol: Symbol) -> bool }
```

## Visibility

- `pub`

## Docstring

Replace `symbol` in the container — matches by `symbol.uuid`.
Returns `false` when the uuid is not present (caller should
`push` into `symbols` instead).

## Source
Lines 682–690 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
