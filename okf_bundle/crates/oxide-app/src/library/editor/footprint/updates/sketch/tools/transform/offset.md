---
okf_version: "0.2"
type: Function
title: offset
description: "v0.22 Phase B2 — Offset tool. Pre-condition: a Line / Arc / Circle is"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset
language: rust
---

# offset

v0.22 Phase B2 — Offset tool. Pre-condition: a Line / Arc / Circle is

## Signature

```rust
fn offset(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Docstring

v0.22 Phase B2 — Offset tool. Pre-condition: a Line / Arc / Circle is
in `selected_sketch`. The click position determines which side of the
source curve the offset lands on. Offset distance comes from
`state.placement_input` (kind OffsetDistance) when the user typed
one; else `state.dimension_input`; else DEFAULT_TOOL_DIMENSION_MM —
and only when both buffers are empty (#599).

## Source
Lines 284–366 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [resolve_tool_dimension_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_tool_dimension_mm.md) |
| calls | [reject_tool_dimension](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/reject_tool_dimension.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [offset_line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_line.md) |
| calls | [offset_circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_circle.md) |
| calls | [offset_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset_arc.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
