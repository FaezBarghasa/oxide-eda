---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/apply
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
Lines 26–37 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [edit](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit.md) |
| calls | [fillet](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/fillet.md) |
| calls | [trim](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/trim.md) |
| calls | [break_track](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/edit/break_track.md) |
