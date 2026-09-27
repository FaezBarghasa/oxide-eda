---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/apply
language: rust
---

# apply

## Signature

```rust
pub(in crate::library::editor::footprint::updates) fn apply(
    editor: &mut crate::app::FootprintEditorState,
    msg: FootprintEditorMsg,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::updates)`

## Source
Lines 9–23 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui.md) |
| calls | [select_many](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select_many.md) |
| calls | [set_tool](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/set_tool.md) |
| calls | [toggle_construction](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/toggle_construction.md) |
| calls | [toggle_centerline](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/toggle_centerline.md) |
| calls | [tool_escape](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/tool_escape.md) |
| calls | [select](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/select.md) |
| calls | [dimension_input](/crates/oxide-app/src/library/editor/footprint/updates/sketch/ui/dimension_input.md) |
