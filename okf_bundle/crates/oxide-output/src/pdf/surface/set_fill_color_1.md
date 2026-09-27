---
okf_version: "0.2"
type: Function
title: set_fill_color
description: "Set non-stroking color (fill/text). Emits `rg` only if changed."
resource: crates/oxide-output/src/pdf/surface.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/pdf/surface/set_fill_color_1
language: rust
---

# set_fill_color

Set non-stroking color (fill/text). Emits `rg` only if changed.

## Signature

```rust
pub fn set_fill_color(&mut self, r: f32, g: f32, b: f32)
```

## Visibility

- `pub`

## Docstring

Set non-stroking color (fill/text). Emits `rg` only if changed.

## Source
Lines 68–78 in `crates/oxide-output/src/pdf/surface.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [surface](/crates/oxide-output/src/pdf/surface.md) |
