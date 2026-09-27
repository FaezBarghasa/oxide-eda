---
okf_version: "0.2"
type: Function
title: placement_field_buf
description: "v0.14-footprint — a placement-input field's raw buffer by kind,"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/placement_field_buf
language: rust
---

# placement_field_buf

v0.14-footprint — a placement-input field's raw buffer by kind,

## Signature

```rust
fn placement_field_buf(
    state: &FootprintEditorState,
    kind: crate::library::editor::footprint::state::PlacementInputKind,
) -> Option<&str>
```

## Docstring

v0.14-footprint — a placement-input field's raw buffer by kind,
searched across the focused slot then the parked fields. The live
dimension pills use it so each shows the user's typed digits
verbatim (never reformatted mid-type).

## Source
Lines 88–97 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [draw_sketch_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/preview/draw_sketch_tool_preview.md) |
