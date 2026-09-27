---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/apply
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
Lines 22–34 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform.md) |
| calls | [mirror](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/mirror.md) |
| calls | [offset](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/offset.md) |
| calls | [rect_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/rect_pattern.md) |
| calls | [circular_pattern](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/transform/circular_pattern.md) |
