---
okf_version: "0.2"
type: Function
title: row_to_label
resource: crates/oxide-types/src/format/sch_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/sch_rows/row_to_label
language: rust
---

# row_to_label

## Signature

```rust
pub(in crate::format) fn row_to_label(row: SchLabelRow) -> Label
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 399–414 in `crates/oxide-types/src/format/sch_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sch_rows](/crates/oxide-types/src/format/sch_rows.md) |
| calls | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
| calls | [parse_label_kind](/crates/oxide-types/src/format/sch_rows/parse_label_kind.md) |
| calls | [parse_halign](/crates/oxide-types/src/format/sch_rows/parse_halign.md) |
| calls | [parse_valign](/crates/oxide-types/src/format/sch_rows/parse_valign.md) |
