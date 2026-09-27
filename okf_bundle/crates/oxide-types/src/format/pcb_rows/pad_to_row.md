---
okf_version: "0.2"
type: Function
title: pad_to_row
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/pad_to_row
language: rust
---

# pad_to_row

## Signature

```rust
pub(in crate::format) fn pad_to_row(pad: &Pad, footprint_ref: &str) -> PcbPadRow
```

## Visibility

- `pub(in crate::format)`

## Source
Lines 382–409 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| calls | [mm_to_nm](/crates/oxide-types/src/format/units/mm_to_nm.md) |
| calls | [pad_type_str](/crates/oxide-types/src/format/pcb_rows/pad_type_str.md) |
| calls | [pad_shape_str](/crates/oxide-types/src/format/pcb_rows/pad_shape_str.md) |
| calls | [join_layers](/crates/oxide-types/src/format/pcb_rows/join_layers.md) |
| called_by | [write_string](/crates/oxide-types/src/format/mod/write_string.md) |
