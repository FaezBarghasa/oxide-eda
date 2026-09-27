---
okf_version: "0.2"
type: Function
title: add_text_frame
description: v0.14 — Place Text Frame press-drag-release commit (item
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text_frame
language: rust
---

# add_text_frame

v0.14 — Place Text Frame press-drag-release commit (item

## Signature

```rust
fn add_text_frame(
    editor: &mut crate::app::FootprintEditorState,
    x_mm: f64,
    y_mm: f64,
    w_mm: f64,
    h_mm: f64,
)
```

## Docstring

v0.14 — Place Text Frame press-drag-release commit (item
③). Fires once, on release, with the anchor (min corner)
and drag size already resolved by the canvas. Pushes its
own history snapshot — see `mutates_footprint_state`,
classified alongside the 3D Body mint variants — because
the intermediate press/drag ticks never reach the
dispatcher (unlike Track's 2-click gesture), so there's
no risk of double-stacking.

## Source
Lines 266–281 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
