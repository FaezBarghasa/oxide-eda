---
okf_version: "0.2"
type: Module
title: rows
description: "Pure, declarative row data for the symbol-editor right-click"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows
language: rust
---

# rows

Pure, declarative row data for the symbol-editor right-click

## Docstring

Pure, declarative row data for the symbol-editor right-click
context menu (data-to-menu, mirrors the app-level context-menu
layer's `Vec<DropdownEntry>` pattern in
`app/view/context_menu/items.rs`).

[`build_symbol_context_menu_rows`] computes WHAT the menu contains
as a `Vec<SymbolMenuRow>` — a pure function of the active symbol's
graphics plus the current selection, no iced dependency, no
rendering. `super::flatten` is the one place that walks this data
into widgets.

## Relationships

| Type | Target |
|------|--------|
| related | [SymbolMenuRow](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/SymbolMenuRow.md) |
| related | [item](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/item.md) |
| related | [submenu](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/submenu.md) |
| related | [item](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/item.md) |
| related | [submenu](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/submenu.md) |
| related | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
| related | [place_submenu](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/place_submenu.md) |
| related | [row_ids](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/row_ids.md) |
| related | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| related | [top_level_ids_are_stable](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/top_level_ids_are_stable.md) |
| related | [empty_selection_disables_selection_dependent_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/empty_selection_disables_selection_dependent_rows.md) |
| related | [all_selection_disables_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_disables_delete.md) |
| related | [line_only_selection_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/line_only_selection_enables_join.md) |
| related | [single_line_selection_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_line_selection_disables_join_but_not_delete.md) |
| related | [single_arc_selection_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_arc_selection_enables_join.md) |
| related | [all_selection_with_a_joinable_ring_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_with_a_joinable_ring_enables_join.md) |
| related | [mixed_selection_with_rectangle_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/mixed_selection_with_rectangle_disables_join_but_not_delete.md) |
| related | [polygon_selected_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/polygon_selected_disables_join_but_not_delete.md) |
