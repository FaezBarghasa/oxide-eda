---
okf_version: "0.2"
type: Class
title: Pin
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
concept_id: crates/oxide-types/src/schematic/mod/Pin
language: rust
---

# Pin

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct Pin
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `direction`
- `shape_style`
- `position`
- `rotation`
- `length`
- `name`
- `number`
- `visible`
- `name_visible`
- `number_visible`

## Source
Lines 610–628 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
| called_by | [sym_editor_select_pin](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_select_pin.md) |
| called_by | [symbol_context_target_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_context_target_to_msg.md) |
| called_by | [symbol_selection_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_selection_to_msg.md) |
| called_by | [on_secondary_release](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/on_secondary_release.md) |
| called_by | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test.md) |
| called_by | [delete_pin_clears_selection_via_return](/crates/oxide-app/src/library/editor/symbol/state/tests/delete_pin_clears_selection_via_return.md) |
| called_by | [move_selected_updates_position](/crates/oxide-app/src/library/editor/symbol/state/tests/move_selected_updates_position.md) |
| called_by | [rotate_selected_rotates_pin_orientation_in_place](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_rotates_pin_orientation_in_place.md) |
| called_by | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| called_by | [context_menu_action_applies_inner_and_closes_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_action_applies_inner_and_closes_menu.md) |
| called_by | [show_context_menu_on_empty_leaves_selection_untouched](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_empty_leaves_selection_untouched.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [context_target_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/context_target_msg_to_state.md) |
| called_by | [apply_symbol_selection](/crates/oxide-app/src/library/editor/symbol/updates/selection/apply_symbol_selection.md) |
| called_by | [align_selected_to_grid_snaps_the_selected_pin_and_dirties_once](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_snaps_the_selected_pin_and_dirties_once.md) |
| called_by | [align_selected_to_grid_with_already_aligned_pin_stays_clean](/crates/oxide-app/src/library/editor/symbol/updates/transform/align_selected_to_grid_with_already_aligned_pin_stays_clean.md) |
| called_by | [evaluate_rule](/crates/oxide-erc-dsl/src/compiler/evaluate_rule.md) |
| called_by | [subject_ref](/crates/oxide-erc-dsl/src/compiler/subject_ref.md) |
