---
okf_version: "0.2"
type: Function
title: is_approximately_white
description: "Check if an RGB value is approximately white (all channels > 0.9)."
resource: crates/oxide-output/src/pdf/colour.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/colour/is_approximately_white
language: rust
---

# is_approximately_white

Check if an RGB value is approximately white (all channels > 0.9).

## Signature

```rust
fn is_approximately_white(r: f32, g: f32, b: f32) -> bool
```

## Docstring

Check if an RGB value is approximately white (all channels > 0.9).

## Source
Lines 73–75 in `crates/oxide-output/src/pdf/colour.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [colour](/crates/oxide-output/src/pdf/colour.md) |
| called_by | [map_rgb](/crates/oxide-output/src/pdf/colour/map_rgb.md) |
