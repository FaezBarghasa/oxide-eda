---
okf_version: "0.2"
type: Function
title: add_pad
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/add_pad
language: rust
---

# add_pad

## Signature

```rust
fn add_pad(editor: &mut crate::app::FootprintEditorState, x_mm: f64, y_mm: f64)
```

## Source
Lines 53–72 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| calls | [footprint_sketch_is_active](/crates/oxide-app/src/library/editor/footprint/updates/geometry/footprint_sketch_is_active.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
