---
okf_version: "0.2"
type: Function
title: set_pads_tool
resource: crates/oxide-app/src/library/editor/footprint/updates/view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/view/set_pads_tool
language: rust
---

# set_pads_tool

## Signature

```rust
fn set_pads_tool(
    editor: &mut crate::app::FootprintEditorState,
    tool: crate::library::editor::footprint::state::PadsTool,
)
```

## Source
Lines 105–174 in `crates/oxide-app/src/library/editor/footprint/updates/view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/library/editor/footprint/updates/view.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/view/apply.md) |
