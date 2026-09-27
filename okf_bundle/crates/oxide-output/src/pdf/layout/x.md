---
okf_version: "0.2"
type: Function
title: x
description: Map a schematic X coordinate to output units.
resource: crates/oxide-output/src/pdf/layout.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/layout/x
language: rust
---

# x

Map a schematic X coordinate to output units.

## Signature

```rust
impl PageTransform { pub fn x(&self, sch_x: f64) -> f32 }
```

## Visibility

- `pub`

## Docstring

Map a schematic X coordinate to output units.
[inline]

## Source
Lines 123–125 in `crates/oxide-output/src/pdf/layout.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [layout](/crates/oxide-output/src/pdf/layout.md) |
