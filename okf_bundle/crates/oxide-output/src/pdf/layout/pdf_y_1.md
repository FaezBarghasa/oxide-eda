---
okf_version: "0.2"
type: Function
title: pdf_y
description: "Map a schematic Y coordinate to a **PDF Y** coordinate."
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/pdf_y_1
language: rust
---

# pdf_y

Map a schematic Y coordinate to a **PDF Y** coordinate.

## Signature

```rust
pub fn pdf_y(&self, sch_y: f64, page_h_units: f32) -> f32
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Map a schematic Y coordinate to a **PDF Y** coordinate.

PDF origin is bottom-left, Y increases upward, so we flip.
[inline]

## Source
Lines 131–133 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
