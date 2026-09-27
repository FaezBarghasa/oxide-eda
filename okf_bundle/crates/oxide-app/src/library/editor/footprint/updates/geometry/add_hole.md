---
okf_version: "0.2"
type: Function
title: add_hole
description: v0.18.12 — Place Hole tool. Drops a non-plated through
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/add_hole
language: rust
---

# add_hole

v0.18.12 — Place Hole tool. Drops a non-plated through

## Signature

```rust
fn add_hole(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Docstring

v0.18.12 — Place Hole tool. Drops a non-plated through
hole at the cursor (no copper, drill from `next_pad_defaults`).

## Source
Lines 285–297 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| calls | [footprint_sketch_is_active](/crates/oxide-app/src/library/editor/footprint/updates/geometry/footprint_sketch_is_active.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
