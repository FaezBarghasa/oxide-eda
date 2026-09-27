---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/apply
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
Lines 156–168 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tools](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod.md) |
| calls | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
| called_by | [handle_tool_click](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/mod/handle_tool_click.md) |
