---
okf_version: "0.2"
type: Function
title: map_rgb
description: Transform an RGB triple (each in range 0.0–1.0) to the target colour mode.
resource: crates/oxide-output/src/pdf/colour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/colour/map_rgb
language: rust
---

# map_rgb

Transform an RGB triple (each in range 0.0–1.0) to the target colour mode.

## Signature

```rust
impl ColourMap { pub fn map_rgb(&self, r: f32, g: f32, b: f32) -> (f32, f32, f32) }
```

## Visibility

- `pub`

## Docstring

Transform an RGB triple (each in range 0.0–1.0) to the target colour mode.

## Source
Lines 22–42 in `crates/oxide-output/src/pdf/colour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [colour](/crates/oxide-output/src/pdf/colour.md) |
| calls | [is_approximately_white](/crates/oxide-output/src/pdf/colour/is_approximately_white.md) |
