---
okf_version: "0.2"
type: Function
title: offset_arc
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_arc
language: rust
---

# offset_arc

## Signature

```rust
fn offset_arc(
    editor: &mut crate::app::FootprintEditorState,
    ctx: &ToolClickCtx,
    source_id: SketchEntityId,
    center: SketchEntityId,
    start: SketchEntityId,
    end: SketchEntityId,
    sweep_ccw: bool,
    dist: f64,
)
```

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "a sketch transform takes the full source geometry plus the transform spec"
)`

## Source
Lines 591–689 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
