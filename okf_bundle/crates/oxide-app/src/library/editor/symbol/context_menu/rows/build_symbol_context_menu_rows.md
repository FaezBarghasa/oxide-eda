---
okf_version: "0.2"
type: Function
title: build_symbol_context_menu_rows
description: Build the full row tree for the current selection. Every row is
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows
language: rust
---

# build_symbol_context_menu_rows

Build the full row tree for the current selection. Every row is

## Signature

```rust
pub fn build_symbol_context_menu_rows(
    sym: &Symbol,
    active_part: u8,
    selected: &Option<SymbolSelection>,
) -> Vec<SymbolMenuRow>
```

## Visibility

- `pub`

## Docstring

Build the full row tree for the current selection. Every row is
present unconditionally — selection-awareness lives entirely in
`enabled` (not row presence), so callers/tests get a stable id
list regardless of selection state; the renderer greys out /
disables clicks on a `enabled: false` row.

## Source
Lines 62–100 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| called_by | [view_context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/view_context_menu.md) |
| called_by | [all_selection_disables_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_disables_delete.md) |
| called_by | [all_selection_with_a_joinable_ring_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/all_selection_with_a_joinable_ring_enables_join.md) |
| called_by | [empty_selection_disables_selection_dependent_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/empty_selection_disables_selection_dependent_rows.md) |
| called_by | [line_only_selection_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/line_only_selection_enables_join.md) |
| called_by | [mixed_selection_with_rectangle_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/mixed_selection_with_rectangle_disables_join_but_not_delete.md) |
| called_by | [polygon_selected_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/polygon_selected_disables_join_but_not_delete.md) |
| called_by | [single_arc_selection_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_arc_selection_enables_join.md) |
| called_by | [single_line_selection_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_line_selection_disables_join_but_not_delete.md) |
| called_by | [top_level_ids_are_stable](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/top_level_ids_are_stable.md) |
