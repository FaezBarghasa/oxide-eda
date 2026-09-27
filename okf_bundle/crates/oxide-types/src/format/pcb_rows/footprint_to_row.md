---
okf_version: "0.2"
type: Function
title: footprint_to_row
description: "---------------------------------------------------------------------------"
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/footprint_to_row
language: rust
---

# footprint_to_row

---------------------------------------------------------------------------

## Signature

```rust
pub(in crate::format) fn footprint_to_row(fp: &Footprint) -> PcbFootprintRow
```

## Visibility

- `pub(in crate::format)`

## Docstring

---------------------------------------------------------------------------
Footprint / Pad / Segment / Via translation
---------------------------------------------------------------------------

## Source
Lines 343–354 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
| calls | [mm_to_nm](/crates/oxide-types/src/format/units/mm_to_nm.md) |
