---
okf_version: "0.2"
type: Function
title: resolve_tool_dimension_mm
description: "Read a tool's millimetre dimension: the cursor-overlay"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/resolve_tool_dimension_mm
language: rust
---

# resolve_tool_dimension_mm

Read a tool's millimetre dimension: the cursor-overlay

## Signature

```rust
pub(super) fn resolve_tool_dimension_mm(
    editor: &crate::app::FootprintEditorState,
    kind: crate::library::editor::footprint::state::PlacementInputKind,
) -> ToolDimension
```

## Visibility

- `pub(super)`

## Docstring

Read a tool's millimetre dimension: the cursor-overlay
`placement_input` of `kind` first, then the Properties-panel
`dimension_input`, then [`DEFAULT_TOOL_DIMENSION_MM`].

GH #599 — a buffer the user actually typed into is never replaced by
the default. Doing so does not merely lose the operation, it
*succeeds* at a size nobody asked for: a `1,5` typed on a
comma-decimal keyboard used to mint a 0.5 mm fillet, and a wrong
fillet radius goes to fabrication. Only an absent or empty buffer
falls through to the default.

## Source
Lines 74–113 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [parse_positive_mm](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/parse_positive_mm.md) |
| called_by | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
| called_by | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
