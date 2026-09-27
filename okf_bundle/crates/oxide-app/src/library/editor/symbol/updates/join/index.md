# join

## Functions

- [all_part0_selection_keeps_part_zero_on_the_result](all_part0_selection_keeps_part_zero_on_the_result.md) — A selection whose sources are all shared (part 0) keeps the
- [all_selection_resolves_to_every_visible_graphic_and_joins](all_selection_resolves_to_every_visible_graphic_and_joins.md) — `SymbolSelection::All` (box-select-everything / Ctrl+A / the
- [apply_symbol_join](apply_symbol_join.md)
- [arc_and_lines_join](arc_and_lines_join.md) — A triangle built from two lines and one arc side joins fine.
- [branching_selection_errors_with_no_mutation](branching_selection_errors_with_no_mutation.md) — A branching (T-junction) selection errors, mutates nothing, and
- [chain_error_message](chain_error_message.md) — Human-readable status-line text for a failed join attempt.
- [direct_closed_join_leaves_status_none](direct_closed_join_leaves_status_none.md) — A selection that was ALREADY a closed chain (no auto-close
- [empty_selection_is_a_no_op](empty_selection_is_a_no_op.md) — Empty selection is a no-op.
- [mixed_shared_and_unit_specific_selection_is_ineligible](mixed_shared_and_unit_specific_selection_is_ineligible.md) — A selection mixing a shared (part 0) source with an
- [new_editor](new_editor.md)
- [open_three_side_chain_auto_closes](open_three_side_chain_auto_closes.md) — 3 of the 4 sides selected (open chain) auto-closes via the
- [push_line](push_line.md)
- [resolve_ring_with_auto_close](resolve_ring_with_auto_close.md) — Chain `segments` into a closed ring, auto-closing exactly once by
- [segments_for](segments_for.md) — Build the `ChainSegment`s + max source stroke width for `indices`.
- [selection_with_rectangle_is_a_no_op](selection_with_rectangle_is_a_no_op.md) — A selection containing a non-Line/Arc graphic (Rectangle) is a
- [single_arc_selection_is_eligible_and_self_closes](single_arc_selection_is_eligible_and_self_closes.md) — A single selected `Arc` IS eligible — a sufficiently large
- [single_line_selection_is_ineligible_and_silent](single_line_selection_is_ineligible_and_silent.md) — A single selected `Line` is ineligible — it can never close on
- [splice_selection_into_polygon](splice_selection_into_polygon.md) — Composite mutation on success: one undo snapshot, remove the
- [square_editor](square_editor.md)
- [square_from_four_lines_joins_and_undo_restores_sources](square_from_four_lines_joins_and_undo_restores_sources.md) — Square from 4 selected lines joins into 1 polygon; the 4
- [stroke_width_is_max_of_sources](stroke_width_is_max_of_sources.md) — The joined polygon's stroke width is the max of the source
