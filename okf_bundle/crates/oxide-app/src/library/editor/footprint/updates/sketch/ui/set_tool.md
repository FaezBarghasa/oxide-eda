---
okf_version: "0.2"
type: Function
title: set_tool
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/set_tool
language: rust
---

# set_tool

## Signature

```rust
fn set_tool(
    editor: &mut crate::app::FootprintEditorState,
    tool: crate::library::editor::footprint::state::SketchTool,
)
```

## Source
Lines 45–52 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/apply.md) |
