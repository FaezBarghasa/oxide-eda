---
okf_version: "0.2"
type: Function
title: items
description: Build the Active Bar items for the given editor state. Theme is
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items
language: rust
---

# items

Build the Active Bar items for the given editor state. Theme is

## Signature

```rust
pub fn items(
    editor: &'a FootprintEditorState,
    theme_id: oxide_types::theme::ThemeId,
    tokens: &'a ThemeTokens,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the Active Bar items for the given editor state. Theme is
pulled from `editor.path` → `themes::current_id()` lookup at the
caller's site (same pattern as the SchLib editor).

## Source
Lines 53–357 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.md) |
| calls | [constraint_enable_matrix](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/constraint_enable_matrix.md) |
| calls | [tag_index](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/tag_index.md) |
| calls | [sketch_tool_icon](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/sketch_tool_icon.md) |
| calls | [build_dimension_input](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/build_dimension_input.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/view.md) |
