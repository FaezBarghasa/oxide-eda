---
okf_version: "0.2"
type: Class
title: SymbolFile
description: "Multi-symbol `.snxsym` container — Altium SchLib parity. One file"
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/SymbolFile
language: rust
---

# SymbolFile

Multi-symbol `.snxsym` container — Altium SchLib parity. One file

## Signature

```rust
pub struct SymbolFile
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Multi-symbol `.snxsym` container — Altium SchLib parity. One file
holds many symbols; each symbol still has its own UUID for
`PrimitiveRef` resolution.

Wire format (v0.18.4): TOML manifest header + one `[[symbols]]`
array entry per Symbol. Each entry's bulk pin list is embedded as
a TSV literal multi-line string (`pins_tsv = '''…'''`) — line-
diffable in git, editable in any spreadsheet. Graphics, parameter
maps, and per-symbol metadata stay as inline TOML since they're
either variant-shaped or sparse.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `format`
- `file_uuid`
- `display_name`
- `symbols`
- `created`
- `updated`

## Source
Lines 443–464 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
