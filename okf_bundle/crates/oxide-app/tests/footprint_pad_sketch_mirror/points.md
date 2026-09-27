---
okf_version: "0.2"
type: Function
title: points
description: "Every `Point` in the sketch, as `(id, x, y)`."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/points
language: rust
---

# points

Every `Point` in the sketch, as `(id, x, y)`.

## Signature

```rust
fn points(sketch: &SketchData) -> Vec<(SketchEntityId, f64, f64)>
```

## Docstring

Every `Point` in the sketch, as `(id, x, y)`.

## Source
Lines 63–72 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| called_by | [issue142_reopened_pad_still_moves_its_whole_outline](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline.md) |
