---
okf_version: "0.2"
type: Function
title: apply
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/apply
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
Lines 9–23 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement.md) |
| calls | [input_char](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_char.md) |
| calls | [input_backspace](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_backspace.md) |
| calls | [input_enter](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_enter.md) |
| calls | [input_escape](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_escape.md) |
| calls | [input_tab](/crates/oxide-app/src/library/editor/footprint/updates/sketch/placement/input_tab.md) |
