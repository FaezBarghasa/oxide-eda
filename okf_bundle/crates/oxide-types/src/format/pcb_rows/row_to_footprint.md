---
okf_version: "0.2"
type: Function
title: row_to_footprint
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/row_to_footprint
language: rust
---

# row_to_footprint

## Signature

```rust
pub(in crate::format) fn row_to_footprint(
    row: PcbFootprintRow,
    extras: FootprintExtras,
) -> Footprint
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 356–380 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| calls | [nm_to_mm](/crates/oxide-types/src/format/units/nm_to_mm.md) |
| called_by | [parse](/crates/oxide-types/src/format/mod/parse.md) |
