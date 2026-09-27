---
okf_version: "0.2"
type: Function
title: point_world
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/point_world
language: rust
---

# point_world

## Signature

```rust
fn point_world(
        id: SketchEntityId,
        sketch: &oxide_sketch::SketchData,
        state: &FootprintEditorState,
    ) -> Option<(f64, f64)>
```

## Source
Lines 28–53 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [draw_sketch_overlay](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/entities/draw_sketch_overlay.md) |
