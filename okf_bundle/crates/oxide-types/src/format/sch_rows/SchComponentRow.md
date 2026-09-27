---
okf_version: "0.2"
type: Class
title: SchComponentRow
description: "Bulk row for one [`Symbol`] in the `[sheets.components]` block."
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/SchComponentRow
language: rust
---

# SchComponentRow

Bulk row for one [`Symbol`] in the `[sheets.components]` block.

## Signature

```rust
pub struct SchComponentRow
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Bulk row for one [`Symbol`] in the `[sheets.components]` block.

Captures the fields with one cell per concept (ref designator,
library id, position in nanometres, rotation in degrees, value,
MPN). Symbol-level fields that don't fit a flat row — `fields`
map, `custom_properties`, `pin_uuids`, `instances`, `ref_text` /
`val_text` text-prop overrides — survive in the
`[sheets.component_extras.<uuid>]` auxiliary TOML tables.
[derive(Debug, Clone, PartialEq)]

## Methods

- `uuid`
- `ref_des`
- `library`
- `pos_x`
- `pos_y`
- `rotation`
- `value`
- `mpn`

## Source
Lines 28–37 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
