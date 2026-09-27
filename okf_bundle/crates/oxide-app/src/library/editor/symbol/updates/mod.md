---
okf_version: "0.2"
type: Module
title: updates
description: Update logic for the standalone Symbol editor.
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod
language: rust
---

# updates

Update logic for the standalone Symbol editor.

## Docstring

Update logic for the standalone Symbol editor.

[`apply_symbol_primitive_edit`] is a thin routing table over
[`SymbolEditorMsg`]: each symbol-mutating variant is dispatched to
the concern module that owns it — [`ui`], [`selection`], [`movement`],
[`transform`], [`camera`], [`parts`], or [`history`]. Graphics-placement
variants are handled inline. Undo/redo, drag coalescing, and the
shared `SymEditor` mutators live here so every concern shares one
implementation.

## Relationships

| Type | Target |
|------|--------|
| related | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| related | [push_undo_snapshot](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo_snapshot.md) |
| related | [begin_drag_if_needed](/crates/oxide-app/src/library/editor/symbol/updates/mod/begin_drag_if_needed.md) |
| related | [mark_dirty](/crates/oxide-app/src/library/editor/symbol/updates/mod/mark_dirty.md) |
| related | [clear_stale_status_message](/crates/oxide-app/src/library/editor/symbol/updates/mod/clear_stale_status_message.md) |
| related | [close_pickers](/crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers.md) |
| related | [push_graphic](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic.md) |
| related | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
| related | [normalize_polygon_ring](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_polygon_ring.md) |
| related | [collapse_consecutive_duplicate_vertices](/crates/oxide-app/src/library/editor/symbol/updates/mod/collapse_consecutive_duplicate_vertices.md) |
| related | [dist_sq](/crates/oxide-app/src/library/editor/symbol/updates/mod/dist_sq.md) |
| related | [polygon_is_collinear](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_is_collinear.md) |
| related | [normalize_arc_commit_deg](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg.md) |
| related | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| related | [context_target_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/context_target_msg_to_state.md) |
| related | [context_submenu_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/context_submenu_msg_to_state.md) |
| related | [symbol_bbox](/crates/oxide-app/src/library/editor/symbol/updates/mod/symbol_bbox.md) |
| related | [graphic_handle_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/graphic_handle_msg_to_state.md) |
| related | [rotate_pivot_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/rotate_pivot_msg_to_state.md) |
| related | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| related | [stale_status_message_clears_on_next_mutating_message](/crates/oxide-app/src/library/editor/symbol/updates/mod/stale_status_message_clears_on_next_mutating_message.md) |
| related | [status_message_survives_camera_pan](/crates/oxide-app/src/library/editor/symbol/updates/mod/status_message_survives_camera_pan.md) |
| related | [polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry.md) |
| related | [polygon_commit_with_fewer_than_three_vertices_is_discarded](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_fewer_than_three_vertices_is_discarded.md) |
| related | [polygon_commit_with_collinear_vertices_is_discarded](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_collinear_vertices_is_discarded.md) |
| related | [polygon_commit_drops_duplicate_closing_vertex](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_drops_duplicate_closing_vertex.md) |
| related | [polygon_commit_with_self_intersecting_bowtie_commits](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_self_intersecting_bowtie_commits.md) |
| related | [polygon_commit_collapses_a_consecutive_duplicate_mid_sequence](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_collapses_a_consecutive_duplicate_mid_sequence.md) |
| related | [polygon_cancel_discards_without_committing](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_cancel_discards_without_committing.md) |
| related | [normalize_arc_commit_deg_swaps_a_cw_dragged_pair](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg_swaps_a_cw_dragged_pair.md) |
| related | [normalize_arc_commit_deg_leaves_ccw_pairs_unswapped](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg_leaves_ccw_pairs_unswapped.md) |
| related | [add_arc_commit_stores_swapped_endpoints_for_a_cw_drag](/crates/oxide-app/src/library/editor/symbol/updates/mod/add_arc_commit_stores_swapped_endpoints_for_a_cw_drag.md) |
| related | [arc_sweep_rejected_sets_status_message_without_committing](/crates/oxide-app/src/library/editor/symbol/updates/mod/arc_sweep_rejected_sets_status_message_without_committing.md) |
