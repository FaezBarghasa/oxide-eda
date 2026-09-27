---
okf_version: "0.2"
type: Function
title: offset_line
description: "Offset a Line: emits a parallel Line at perpendicular distance and"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_line
language: rust
---

# offset_line

Offset a Line: emits a parallel Line at perpendicular distance and

## Signature

```rust
fn offset_line(
    editor: &mut crate::app::FootprintEditorState,
    ctx: &ToolClickCtx,
    source_id: SketchEntityId,
    start: SketchEntityId,
    end: SketchEntityId,
    dist: f64,
)
```

## Docstring

Offset a Line: emits a parallel Line at perpendicular distance and
adds (Parallel + DistancePtLine) constraints so the relationship
survives source edits.

## Source
Lines 371–482 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
