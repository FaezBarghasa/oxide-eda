---
okf_version: "0.2"
type: Function
title: fillet_second_click
description: "Second click — pick the second Line, compute the tangent geometry, and"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click
language: rust
---

# fillet_second_click

Second click — pick the second Line, compute the tangent geometry, and

## Signature

```rust
fn fillet_second_click(
    editor: &mut crate::app::FootprintEditorState,
    ctx: &ToolClickCtx,
    click_xy: (f64, f64),
    first_line: SketchEntityId,
    radius_mm: f64,
)
```

## Docstring

Second click — pick the second Line, compute the tangent geometry, and
splice in the fillet Arc.

## Source
Lines 138–368 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [pick_line_at](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/pick_line_at.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
