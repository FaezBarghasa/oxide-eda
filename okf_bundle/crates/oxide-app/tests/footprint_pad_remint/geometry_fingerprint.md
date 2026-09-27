---
okf_version: "0.2"
type: Function
title: geometry_fingerprint
description: "Every `Point` position in the sketch, sorted, plus the per-kind"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/geometry_fingerprint
language: rust
---

# geometry_fingerprint

Every `Point` position in the sketch, sorted, plus the per-kind

## Signature

```rust
fn geometry_fingerprint(sketch: &SketchData) -> (Vec<(i64, i64)>, [usize; 4])
```

## Docstring

Every `Point` position in the sketch, sorted, plus the per-kind
entity census. Entity IDs are freshly generated on every mint, so
two sketches are compared through their geometry rather than their
UUIDs. Positions are compared at nanometre resolution — the unit
oxide coordinates are integral in downstream.

## Source
Lines 96–114 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| called_by | [one_undo_after_a_rotate_restores_the_prior_sketch_geometry](/crates/oxide-app/tests/footprint_pad_remint/one_undo_after_a_rotate_restores_the_prior_sketch_geometry.md) |
