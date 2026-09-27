---
okf_version: "0.2"
type: Function
title: map_stroke_bw
description: "Map a stroke colour for B&W mode — always returns black."
resource: crates/oxide-output/src/pdf/colour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/colour/map_stroke_bw
language: rust
---

# map_stroke_bw

Map a stroke colour for B&W mode — always returns black.

## Signature

```rust
impl ColourMap { pub fn map_stroke_bw(&self, _r: f32, _g: f32, _b: f32) -> (f32, f32, f32) }
```

## Visibility

- `pub`

## Docstring

Map a stroke colour for B&W mode — always returns black.

## Source
Lines 45–51 in `crates/oxide-output/src/pdf/colour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [colour](/crates/oxide-output/src/pdf/colour.md) |
