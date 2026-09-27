---
okf_version: "0.2"
type: Function
title: move_point
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/move_point
language: rust
---

# move_point

## Signature

```rust
fn move_point(
    editor: &mut crate::app::FootprintEditorState,
    id: oxide_sketch::id::SketchEntityId,
    dx: f64,
    dy: f64,
)
```

## Source
Lines 71–186 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entities](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [remint_dragged_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/apply.md) |
