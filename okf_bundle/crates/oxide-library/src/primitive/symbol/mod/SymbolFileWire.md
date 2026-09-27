---
okf_version: "0.2"
type: Class
title: SymbolFileWire
description: "On-disk wire shape. Mirrors [`SymbolFile`] but each [`Symbol`]'s"
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/SymbolFileWire
language: rust
---

# SymbolFileWire

On-disk wire shape. Mirrors [`SymbolFile`] but each [`Symbol`]'s

## Signature

```rust
struct SymbolFileWire
```

## Decorators

- `derive(Serialize, Deserialize)`

## Docstring

On-disk wire shape. Mirrors [`SymbolFile`] but each [`Symbol`]'s
`pins` Vec is replaced with a `pins_tsv: String` carrying the TSV-
encoded payload.
[derive(Serialize, Deserialize)]

## Methods

- `format`
- `file_uuid`
- `display_name`
- `created`
- `updated`
- `symbols`

## Source
Lines 540–549 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
