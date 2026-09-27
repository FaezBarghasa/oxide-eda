---
okf_version: "0.2"
type: Function
title: fillet
description: "v0.27 — EDA Fillet. Two-click gesture:"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet
language: rust
---

# fillet

v0.27 — EDA Fillet. Two-click gesture:

## Signature

```rust
fn fillet(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.27 — EDA Fillet. Two-click gesture:
click 1: pick the first Line (we hit-test for a Line near the click
— fall back to a warning if none).
click 2: pick the second Line that shares an endpoint with the
first. Compute tangent points at radius `r` from the shared corner
along each line, splice in an Arc connecting them centred on the
angle bisector, and shorten both lines to end at the tangent
points.

Radius source — `state.placement_input` (kind FilletRadius) when the
user typed one; else `state.dimension_input`; else
DEFAULT_TOOL_DIMENSION_MM, and only when both buffers are empty.

GH #599 — a typed-but-unreadable radius aborts the gesture with a
report instead of quietly filleting at the default. A fillet radius
the user did not ask for is manufactured geometry that reaches fab.

## Source
Lines 95–112 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [resolve_tool_dimension_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_tool_dimension_mm.md) |
| calls | [reject_tool_dimension](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/reject_tool_dimension.md) |
| calls | [fillet_second_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_second_click.md) |
| calls | [fillet_first_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet_first_click.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/apply.md) |
