---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply
language: rust
---

# apply

## Signature

```rust
pub(super) fn apply(
    editor: &mut crate::app::FootprintEditorState,
    ctx: &ToolClickCtx,
    tool: SketchTool,
)
```

## Visibility

- `pub(super)`

## Source
Lines 21–37 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [select_or_point](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/select_or_point.md) |
| calls | [line](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/line.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
| calls | [rounded_rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle.md) |
| calls | [rectangle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rectangle.md) |
| calls | [arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/arc.md) |
| calls | [edge_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/edge_arc.md) |
| calls | [tangent_arc](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/tangent_arc.md) |
