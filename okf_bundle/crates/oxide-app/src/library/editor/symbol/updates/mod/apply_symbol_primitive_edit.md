---
okf_version: "0.2"
type: Function
title: apply_symbol_primitive_edit
description: Apply a primitive-editor event to a standalone Symbol editor
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit
language: rust
---

# apply_symbol_primitive_edit

Apply a primitive-editor event to a standalone Symbol editor

## Signature

```rust
pub(crate) fn apply_symbol_primitive_edit(
    editor: &mut crate::app::SymbolEditorState,
    msg: SymbolEditorMsg,
)
```

## Visibility

- `pub(crate)`

## Docstring

Apply a primitive-editor event to a standalone Symbol editor
state. Mirrors the symbol-tab arms of `apply_inline_edit` but
against the path-keyed standalone state. Visibility is
`pub(crate)` so unit tests in sibling modules can drive the editor
through the same code path the dispatcher uses.

## Source
Lines 289–464 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [clear_stale_status_message](/crates/oxide-app/src/library/editor/symbol/updates/mod/clear_stale_status_message.md) |
| calls | [apply_symbol_ui](/crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| calls | [add_pin](/crates/oxide-app/src/library/editor/symbol/state/mod/add_pin.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [mark_dirty](/crates/oxide-app/src/library/editor/symbol/updates/mod/mark_dirty.md) |
| calls | [push_graphic](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_graphic.md) |
| calls | [normalize_arc_commit_deg](/crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg.md) |
| calls | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
| calls | [apply_symbol_selection](/crates/oxide-app/src/library/editor/symbol/updates/selection/apply_symbol_selection.md) |
| calls | [apply_symbol_move](/crates/oxide-app/src/library/editor/symbol/updates/movement/apply_symbol_move.md) |
| calls | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
| calls | [apply_symbol_camera](/crates/oxide-app/src/library/editor/symbol/updates/camera/apply_symbol_camera.md) |
| calls | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
| calls | [apply_symbol_history](/crates/oxide-app/src/library/editor/symbol/updates/history/apply_symbol_history.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| called_by | [handle_symbol_primitive_edit](/crates/oxide-app/src/app/dispatch/library/editor/handle_symbol_primitive_edit.md) |
| called_by | [context_menu_action_applies_inner_and_closes_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_action_applies_inner_and_closes_menu.md) |
| called_by | [add_arc_commit_stores_swapped_endpoints_for_a_cw_drag](/crates/oxide-app/src/library/editor/symbol/updates/mod/add_arc_commit_stores_swapped_endpoints_for_a_cw_drag.md) |
| called_by | [arc_sweep_rejected_sets_status_message_without_committing](/crates/oxide-app/src/library/editor/symbol/updates/mod/arc_sweep_rejected_sets_status_message_without_committing.md) |
| called_by | [polygon_cancel_discards_without_committing](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_cancel_discards_without_committing.md) |
| called_by | [polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_click_then_commit_pushes_one_graphic_and_one_undo_entry.md) |
| called_by | [polygon_commit_collapses_a_consecutive_duplicate_mid_sequence](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_collapses_a_consecutive_duplicate_mid_sequence.md) |
| called_by | [polygon_commit_drops_duplicate_closing_vertex](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_drops_duplicate_closing_vertex.md) |
| called_by | [polygon_commit_with_collinear_vertices_is_discarded](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_collinear_vertices_is_discarded.md) |
| called_by | [polygon_commit_with_fewer_than_three_vertices_is_discarded](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_fewer_than_three_vertices_is_discarded.md) |
| called_by | [polygon_commit_with_self_intersecting_bowtie_commits](/crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_self_intersecting_bowtie_commits.md) |
| called_by | [stale_status_message_clears_on_next_mutating_message](/crates/oxide-app/src/library/editor/symbol/updates/mod/stale_status_message_clears_on_next_mutating_message.md) |
| called_by | [status_message_survives_camera_pan](/crates/oxide-app/src/library/editor/symbol/updates/mod/status_message_survives_camera_pan.md) |
| called_by | [polygon_stash_never_crosses_between_two_editors](/crates/oxide-app/src/library/editor/symbol/updates/ui/polygon_stash_never_crosses_between_two_editors.md) |
