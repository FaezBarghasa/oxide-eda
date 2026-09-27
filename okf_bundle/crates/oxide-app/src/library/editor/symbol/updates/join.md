---
okf_version: "0.2"
type: Module
title: join
description: "Symbol editor — \"Join into Polygon\" selection op."
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join
language: rust
---

# join

Symbol editor — "Join into Polygon" selection op.

## Docstring

Symbol editor — "Join into Polygon" selection op.

Chains the currently-selected `Line`/`Arc` graphics end-to-end
(via `oxide_library::chain_into_closed_contour`) into a single
closed `Polygon`, replacing the source graphics. See
[`apply_symbol_join`] for the full contract.

## Relationships

| Type | Target |
|------|--------|
| related | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
| related | [resolve_ring_with_auto_close](/crates/oxide-app/src/library/editor/symbol/updates/join/resolve_ring_with_auto_close.md) |
| related | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| related | [segments_for](/crates/oxide-app/src/library/editor/symbol/updates/join/segments_for.md) |
| related | [chain_error_message](/crates/oxide-app/src/library/editor/symbol/updates/join/chain_error_message.md) |
| related | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| related | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| related | [square_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/square_editor.md) |
| related | [square_from_four_lines_joins_and_undo_restores_sources](/crates/oxide-app/src/library/editor/symbol/updates/join/square_from_four_lines_joins_and_undo_restores_sources.md) |
| related | [open_three_side_chain_auto_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/open_three_side_chain_auto_closes.md) |
| related | [direct_closed_join_leaves_status_none](/crates/oxide-app/src/library/editor/symbol/updates/join/direct_closed_join_leaves_status_none.md) |
| related | [arc_and_lines_join](/crates/oxide-app/src/library/editor/symbol/updates/join/arc_and_lines_join.md) |
| related | [branching_selection_errors_with_no_mutation](/crates/oxide-app/src/library/editor/symbol/updates/join/branching_selection_errors_with_no_mutation.md) |
| related | [selection_with_rectangle_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/selection_with_rectangle_is_a_no_op.md) |
| related | [stroke_width_is_max_of_sources](/crates/oxide-app/src/library/editor/symbol/updates/join/stroke_width_is_max_of_sources.md) |
| related | [empty_selection_is_a_no_op](/crates/oxide-app/src/library/editor/symbol/updates/join/empty_selection_is_a_no_op.md) |
| related | [all_part0_selection_keeps_part_zero_on_the_result](/crates/oxide-app/src/library/editor/symbol/updates/join/all_part0_selection_keeps_part_zero_on_the_result.md) |
| related | [mixed_shared_and_unit_specific_selection_is_ineligible](/crates/oxide-app/src/library/editor/symbol/updates/join/mixed_shared_and_unit_specific_selection_is_ineligible.md) |
| related | [all_selection_resolves_to_every_visible_graphic_and_joins](/crates/oxide-app/src/library/editor/symbol/updates/join/all_selection_resolves_to_every_visible_graphic_and_joins.md) |
| related | [single_line_selection_is_ineligible_and_silent](/crates/oxide-app/src/library/editor/symbol/updates/join/single_line_selection_is_ineligible_and_silent.md) |
| related | [single_arc_selection_is_eligible_and_self_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/single_arc_selection_is_eligible_and_self_closes.md) |
