---
okf_version: "0.2"
type: Function
title: row_to_symbol
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/row_to_symbol
language: rust
---

# row_to_symbol

## Signature

```rust
pub(in crate::format) fn row_to_symbol(row: SchComponentRow, extras: SymbolExtras) -> Symbol
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 289–334 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
| called_by | [parse](/crates/oxide-types/src/format/mod/parse.md) |
