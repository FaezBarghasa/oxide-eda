---
okf_version: "0.2"
type: Class
title: PcbPadRow
description: "Bulk row for one [`Pad`] in the `[pads]` block. The row is keyed"
resource: crates/oxide-types/src/format/pcb_rows.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:12:46Z"
concept_id: crates/oxide-types/src/format/pcb_rows/PcbPadRow
language: rust
---

# PcbPadRow

Bulk row for one [`Pad`] in the `[pads]` block. The row is keyed

## Signature

```rust
pub struct PcbPadRow
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Bulk row for one [`Pad`] in the `[pads]` block. The row is keyed
to its parent footprint by `footprint_ref` (the user-facing
reference designator), keeping the file readable in code-review.
[derive(Debug, Clone, PartialEq)]

## Methods

- `uuid`
- `footprint_ref`
- `pin`
- `pos_x`
- `pos_y`
- `size_x`
- `size_y`
- `pad_type`
- `shape`
- `layers`
- `drill`
- `net_number`
- `net_name`
- `roundrect_ratio`

## Source
Lines 72–87 in `crates/oxide-types/src/format/pcb_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_rows](/crates/oxide-types/src/format/pcb_rows.md) |
