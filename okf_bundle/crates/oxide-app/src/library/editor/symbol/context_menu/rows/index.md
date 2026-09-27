# rows

## Classs

- [SymbolMenuRow](SymbolMenuRow.md) — One row of the symbol context menu. `id` is a stable, kebab-case,

## Functions

- [all_selection_disables_delete](all_selection_disables_delete.md) — `SymbolSelection::All` is a no-op for `delete_selected`, so the
- [all_selection_with_a_joinable_ring_enables_join](all_selection_with_a_joinable_ring_enables_join.md) — `SymbolSelection::All` resolves to every visible graphic, so a
- [build_symbol_context_menu_rows](build_symbol_context_menu_rows.md) — Build the full row tree for the current selection. Every row is
- [empty_selection_disables_selection_dependent_rows](empty_selection_disables_selection_dependent_rows.md) — Empty selection: Join into Polygon, Delete, and Deselect All
- [find](find.md)
- [item](item.md)
- [item](item_1.md)
- [line_only_selection_enables_join](line_only_selection_enables_join.md) — A selection of only Line graphics enables Join into Polygon.
- [mixed_selection_with_rectangle_disables_join_but_not_delete](mixed_selection_with_rectangle_disables_join_but_not_delete.md) — A mixed selection containing a Rectangle disables Join into
- [place_submenu](place_submenu.md) — `Place ▸` submenu built from [`PLACE_TOOLS`]. Always enabled:
- [polygon_selected_disables_join_but_not_delete](polygon_selected_disables_join_but_not_delete.md) — A single selected Polygon can't be joined (it's already the
- [row_ids](row_ids.md)
- [single_arc_selection_enables_join](single_arc_selection_enables_join.md) — A single selected Arc, by contrast, enables Join into Polygon —
- [single_line_selection_disables_join_but_not_delete](single_line_selection_disables_join_but_not_delete.md) — A single selected Line disables Join into Polygon — it can
- [submenu](submenu.md)
- [submenu](submenu_1.md)
- [top_level_ids_are_stable](top_level_ids_are_stable.md) — The top-level id list is stable — the exact "full set" the
