---
okf_version: "0.2"
type: Function
title: offset_circle
description: "Offset a Circle: emits a concentric copy that shares the source's"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_circle
language: rust
---

# offset_circle

Offset a Circle: emits a concentric copy that shares the source's

## Signature

```rust
fn offset_circle(
    editor: &mut crate::app::FootprintEditorState,
    ctx: &ToolClickCtx,
    source_id: SketchEntityId,
    center: SketchEntityId,
    radius: f64,
    dist: f64,
)
```

## Docstring

Offset a Circle: emits a concentric copy that shares the source's
centre Point so the centres stay locked. The new radius is a literal
(source.radius ± dist) — the schema has no radius-dimension
constraint, so further radius edits don't auto-propagate; the user
can re-offset or edit the literal directly.

## Source
Lines 489–581 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
