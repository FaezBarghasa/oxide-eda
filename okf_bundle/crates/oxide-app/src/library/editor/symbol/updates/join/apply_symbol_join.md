---
okf_version: "0.2"
type: Function
title: apply_symbol_join
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join
language: rust
---

# apply_symbol_join

## Signature

```rust
pub(super) fn apply_symbol_join(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 20–76 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [join_source_indices](/crates/oxide-app/src/library/editor/symbol/state/mod/join_source_indices.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| calls | [selection_is_join_eligible](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_is_join_eligible.md) |
| calls | [selection_kinds_are_line_or_arc](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_kinds_are_line_or_arc.md) |
| calls | [selection_has_enough_join_sources](/crates/oxide-app/src/library/editor/symbol/state/mod/selection_has_enough_join_sources.md) |
| calls | [common_graphic_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/common_graphic_part_number.md) |
| calls | [segments_for](/crates/oxide-app/src/library/editor/symbol/updates/join/segments_for.md) |
| calls | [resolve_ring_with_auto_close](/crates/oxide-app/src/library/editor/symbol/updates/join/resolve_ring_with_auto_close.md) |
| calls | [chain_error_message](/crates/oxide-app/src/library/editor/symbol/updates/join/chain_error_message.md) |
| calls | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| called_by | [all_part0_selection_keeps_part_zero_on_the_result](/crates/oxide-app/src/library/editor/symbol/updates/join/all_part0_selection_keeps_part_zero_on_the_result.md) |
| called_by | [all_selection_resolves_to_every_visible_graphic_and_joins](/crates/oxide-app/src/library/editor/symbol/updates/join/all_selection_resolves_to_every_visible_graphic_and_joins.md) |
| called_by | [arc_and_lines_join](/crates/oxide-app/src/library/editor/symbol/updates/join/arc_and_lines_join.md) |
| called_by | [branching_selection_errors_with_no_mutation](/crates/oxide-app/src/library/editor/symbol/updates/join/branching_selection_errors_with_no_mutation.md) |
| called_by | [direct_closed_join_leaves_status_none](/crates/oxide-app/src/library/editor/symbol/updates/join/direct_closed_join_leaves_status_none.md) |
| called_by | [empty_selection_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/empty_selection_is_a_no_op.md) |
| called_by | [mixed_shared_and_unit_specific_selection_is_ineligible](/crates/oxide-app/src/library/editor/symbol/updates/join/mixed_shared_and_unit_specific_selection_is_ineligible.md) |
| called_by | [open_three_side_chain_auto_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/open_three_side_chain_auto_closes.md) |
| called_by | [selection_with_rectangle_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/selection_with_rectangle_is_a_no_op.md) |
| called_by | [single_arc_selection_is_eligible_and_self_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/single_arc_selection_is_eligible_and_self_closes.md) |
| called_by | [single_line_selection_is_ineligible_and_silent](/crates/oxide-app/src/library/editor/symbol/updates/join/single_line_selection_is_ineligible_and_silent.md) |
| called_by | [square_from_four_lines_joins_and_undo_restores_sources](/crates/oxide-app/src/library/editor/symbol/updates/join/square_from_four_lines_joins_and_undo_restores_sources.md) |
| called_by | [stroke_width_is_max_of_sources](/crates/oxide-app/src/library/editor/symbol/updates/join/stroke_width_is_max_of_sources.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
