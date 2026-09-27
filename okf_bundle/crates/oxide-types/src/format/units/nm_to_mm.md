---
okf_version: "0.2"
type: Function
title: nm_to_mm
resource: crates/oxide-types/src/format/units.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/units/nm_to_mm
language: rust
---

# nm_to_mm

## Signature

```rust
pub(in crate::format) fn nm_to_mm(nm: i64) -> f64
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 40–42 in `crates/oxide-types/src/format/units.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [units](/crates/oxide-types/src/format/units.md) |
| called_by | [row_to_footprint](/crates/oxide-types/src/format/pcb_rows/row_to_footprint.md) |
| called_by | [row_to_pad](/crates/oxide-types/src/format/pcb_rows/row_to_pad.md) |
| called_by | [row_to_track](/crates/oxide-types/src/format/pcb_rows/row_to_track.md) |
| called_by | [row_to_via](/crates/oxide-types/src/format/pcb_rows/row_to_via.md) |
| called_by | [row_to_junction](/crates/oxide-types/src/format/sch_rows/row_to_junction.md) |
| called_by | [row_to_label](/crates/oxide-types/src/format/sch_rows/row_to_label.md) |
| called_by | [row_to_symbol](/crates/oxide-types/src/format/sch_rows/row_to_symbol.md) |
| called_by | [row_to_wire](/crates/oxide-types/src/format/sch_rows/row_to_wire.md) |
