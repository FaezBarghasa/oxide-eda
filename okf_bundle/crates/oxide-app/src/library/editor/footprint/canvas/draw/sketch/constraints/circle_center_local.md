---
okf_version: "0.2"
type: Function
title: circle_center_local
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/circle_center_local
language: rust
---

# circle_center_local

## Signature

```rust
fn circle_center_local(
        sketch: &oxide_sketch::SketchData,
        id: oxide_sketch::id::SketchEntityId,
    ) -> Option<oxide_sketch::id::SketchEntityId>
```

## Source
Lines 84–96 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [draw_constraint_icons](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/constraints/draw_constraint_icons.md) |
