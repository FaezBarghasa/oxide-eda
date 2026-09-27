---
okf_version: "0.2"
type: Function
title: overlap_ratio_by_smaller_area
description: "[cfg(test)]"
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/overlap_ratio_by_smaller_area
language: rust
---

# overlap_ratio_by_smaller_area

[cfg(test)]

## Signature

```rust
fn overlap_ratio_by_smaller_area(a: RectPx, b: RectPx) -> f32
```

## Decorators

- `cfg(test)`

## Docstring

[cfg(test)]

## Source
Lines 133–140 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| calls | [rect_area_px](/crates/oxide-gfx/src/pipeline/text/rect_area_px.md) |
| calls | [rect_overlap_area_px](/crates/oxide-gfx/src/pipeline/text/rect_overlap_area_px.md) |
| called_by | [overlap_ratio_detects_dense_overlap](/crates/oxide-gfx/src/pipeline/text/overlap_ratio_detects_dense_overlap.md) |
