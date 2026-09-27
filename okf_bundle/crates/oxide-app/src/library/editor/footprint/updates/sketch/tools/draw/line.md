---
okf_version: "0.2"
type: Function
title: line
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/line
language: rust
---

# line

## Signature

```rust
fn line(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Source
Lines 44–136 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
