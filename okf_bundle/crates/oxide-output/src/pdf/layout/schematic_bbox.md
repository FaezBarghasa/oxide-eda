---
okf_version: "0.2"
type: Function
title: schematic_bbox
description: "Bounding box of all schematic content (wires, symbols, labels) in mm."
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/schematic_bbox
language: rust
---

# schematic_bbox

Bounding box of all schematic content (wires, symbols, labels) in mm.

## Signature

```rust
pub fn schematic_bbox(sheet: &SheetSnapshot) -> (f64, f64, f64, f64)
```

## Visibility

- `pub`

## Docstring

Bounding box of all schematic content (wires, symbols, labels) in mm.

Returns `(x_min, y_min, x_max, y_max)`. Falls back to `(0, 0, 100, 100)`
when the sheet is empty.

## Source
Lines 13–46 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
| called_by | [new](/crates/oxide-output/src/pdf/layout/new.md) |
