---
okf_version: "0.2"
type: Function
title: dropdown_trigger_items
description: Dropdown trigger items for the SchLib bar. Same dual-action
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_trigger_items
language: rust
---

# dropdown_trigger_items

Dropdown trigger items for the SchLib bar. Same dual-action

## Signature

```rust
fn dropdown_trigger_items(
    editor: &SymbolEditorState,
    tid: ThemeId,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Docstring

Dropdown trigger items for the SchLib bar. Same dual-action
pattern as the schematic / footprint bars: left-click runs the
default action (or toggles the menu when there's no obvious
default), right-click opens the dropdown.

## Source
Lines 121–210 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
| called_by | [align_trigger_left_click_snaps_selection_to_grid](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/align_trigger_left_click_snaps_selection_to_grid.md) |
| called_by | [bar_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/bar_items.md) |
| called_by | [move_trigger_left_click_arms_select_tool](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/move_trigger_left_click_arms_select_tool.md) |
