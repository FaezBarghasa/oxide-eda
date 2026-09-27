---
okf_version: "0.2"
type: Function
title: mm_to_nm
description: "HI-12: convert mm → nm without overflow. The unchecked `as i64` cast"
resource: crates/oxide-types/src/format/units.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/units/mm_to_nm
language: rust
---

# mm_to_nm

HI-12: convert mm → nm without overflow. The unchecked `as i64` cast

## Signature

```rust
pub(in crate::format) fn mm_to_nm(mm: f64) -> i64
```

## Visibility

- `pub(in crate::format)`

## Docstring

HI-12: convert mm → nm without overflow. The unchecked `as i64` cast
would wrap to garbage for boards larger than ~9.2 m (and for any
NaN / Inf input). Real PCBs are < 1 m, so we clamp to `i64::MIN/MAX`
rather than panicking — that surfaces a non-finite value as the
largest representable coordinate, which is visibly wrong instead of
silently corrupt.

## Source
Lines 26–38 in `crates/oxide-types/src/format/units.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [units](/crates/oxide-types/src/format/units.md) |
| called_by | [footprint_to_row](/crates/oxide-types/src/format/pcb_rows/footprint_to_row.md) |
| called_by | [pad_to_row](/crates/oxide-types/src/format/pcb_rows/pad_to_row.md) |
| called_by | [track_to_row](/crates/oxide-types/src/format/pcb_rows/track_to_row.md) |
| called_by | [via_to_row](/crates/oxide-types/src/format/pcb_rows/via_to_row.md) |
| called_by | [junction_to_row](/crates/oxide-types/src/format/sch_rows/junction_to_row.md) |
| called_by | [label_to_row](/crates/oxide-types/src/format/sch_rows/label_to_row.md) |
| called_by | [symbol_to_row](/crates/oxide-types/src/format/sch_rows/symbol_to_row.md) |
| called_by | [wire_to_row](/crates/oxide-types/src/format/sch_rows/wire_to_row.md) |
