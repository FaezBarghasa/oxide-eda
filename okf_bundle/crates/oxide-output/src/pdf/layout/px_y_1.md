---
okf_version: "0.2"
type: Function
title: px_y
description: "Map a schematic Y coordinate to a **pixel Y** coordinate."
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/px_y_1
language: rust
---

# px_y

Map a schematic Y coordinate to a **pixel Y** coordinate.

## Signature

```rust
pub fn px_y(&self, sch_y: f64) -> f32
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Map a schematic Y coordinate to a **pixel Y** coordinate.

Pixels origin is top-left, Y increases downward — same as schematic,
no flip needed.
[inline]

## Source
Lines 140–142 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
