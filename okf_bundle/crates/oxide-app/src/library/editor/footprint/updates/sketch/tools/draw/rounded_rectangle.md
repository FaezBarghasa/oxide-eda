---
okf_version: "0.2"
type: Function
title: rounded_rectangle
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/rounded_rectangle
language: rust
---

# rounded_rectangle

## Signature

```rust
fn rounded_rectangle(editor: &mut crate::app::FootprintEditorState, ctx: &ToolClickCtx)
```

## Source
Lines 190–338 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [draw](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| calls | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/apply.md) |
