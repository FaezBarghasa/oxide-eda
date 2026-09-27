---
okf_version: "0.2"
type: Class
title: Graphic
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/Graphic
language: rust
---

# Graphic

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub enum Graphic
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `points`
- `width`
- `fill`
- `start`
- `end`
- `width`
- `fill`
- `center`
- `radius`
- `width`
- `fill`
- `start`
- `mid`
- `end`
- `width`
- `fill`
- `text`
- `position`
- `rotation`
- `font_size`
- `bold`
- `italic`
- `justify_h`
- `justify_v`
- `text`
- `position`
- `rotation`
- `size`
- `font_size`
- `bold`
- `italic`
- `width`
- `fill`
- `points`
- `width`
- `fill`

## Source
Lines 528–603 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
| called_by | [sym_editor_select_graphic](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_graphic.md) |
| called_by | [build_symbol_editor_panel_ctx](/crates/oxide-app/src/app/runtime/symbol_ctx/build_symbol_editor_panel_ctx.md) |
| called_by | [symbol_context_target_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_context_target_to_msg.md) |
| called_by | [symbol_selection_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_selection_to_msg.md) |
| called_by | [on_secondary_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release.md) |
| called_by | [polygon_selected_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/polygon_selected_disables_join_but_not_delete.md) |
| called_by | [single_arc_selection_enables_join](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_arc_selection_enables_join.md) |
| called_by | [single_line_selection_disables_join_but_not_delete](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_line_selection_disables_join_but_not_delete.md) |
| called_by | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test.md) |
| called_by | [delete_selected_removes_graphic](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_selected_removes_graphic.md) |
| called_by | [hit_test_graphic_handle_finds_polygon_vertex_when_selected](/crates/oxide-app/src/library/editor/symbol/state/tests/hit_test_graphic_handle_finds_polygon_vertex_when_selected.md) |
| called_by | [move_selected_translates_polygon_by_centroid_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_polygon_by_centroid_delta.md) |
| called_by | [move_selected_translates_rectangle_by_anchor_delta](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_translates_rectangle_by_anchor_delta.md) |
| called_by | [rotate_selected_about_geometry_center_keeps_rectangle_center](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_rectangle_center.md) |
| called_by | [rotate_selected_about_geometry_center_keeps_text_anchor_fixed](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_text_anchor_fixed.md) |
| called_by | [rotate_selected_about_geometry_center_rotates_polygon_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_rotates_polygon_vertices.md) |
| called_by | [rotate_selected_rotates_rectangle_clockwise_around_origin](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_rectangle_clockwise_around_origin.md) |
| called_by | [rotated_wraparound_arc_hit_test_and_draw_sweep_agree](/crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree.md) |
| called_by | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| called_by | [show_context_menu_on_all_selection_preserves_all](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_all_selection_preserves_all.md) |
| called_by | [show_context_menu_on_already_selected_graphic_is_idempotent](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_already_selected_graphic_is_idempotent.md) |
| called_by | [show_context_menu_on_graphic_selects_it_first](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_graphic_selects_it_first.md) |
| called_by | [show_context_menu_on_multiple_member_preserves_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_member_preserves_selection.md) |
| called_by | [show_context_menu_on_multiple_non_member_replaces_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_non_member_replaces_selection.md) |
| called_by | [single_arc_selection_is_eligible_and_self_closes](/crates/oxide-app/src/library/editor/symbol/updates/join/single_arc_selection_is_eligible_and_self_closes.md) |
| called_by | [single_line_selection_is_ineligible_and_silent](/crates/oxide-app/src/library/editor/symbol/updates/join/single_line_selection_is_ineligible_and_silent.md) |
| called_by | [splice_selection_into_polygon](/crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon.md) |
| called_by | [context_target_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/context_target_msg_to_state.md) |
| called_by | [apply_symbol_selection](/crates/oxide-app/src/library/editor/symbol/updates/selection/apply_symbol_selection.md) |
| called_by | [delete_selected_closes_open_fill_picker](/crates/oxide-app/src/library/editor/symbol/updates/transform/delete_selected_closes_open_fill_picker.md) |
