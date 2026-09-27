---
okf_version: "0.2"
type: Function
title: point_xy
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/point_xy
language: rust
---

# point_xy

## Signature

```rust
fn point_xy(sketch: &SketchData, id: SketchEntityId) -> Option<(f64, f64)>
```

## Source
Lines 74–83 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [issue142_reopened_pad_still_moves_its_whole_outline](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_still_moves_its_whole_outline.md) |
