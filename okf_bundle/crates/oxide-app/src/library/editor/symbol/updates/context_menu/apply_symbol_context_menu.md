---
okf_version: "0.2"
type: Function
title: apply_symbol_context_menu
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu
language: rust
---

# apply_symbol_context_menu

## Signature

```rust
pub(super) fn apply_symbol_context_menu(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 14–60 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [context_target_msg_to_state](/crates/oxide-app/src/library/editor/symbol/updates/mod/context_target_msg_to_state.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [target_in_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/target_in_selection.md) |
| called_by | [close_context_menu_clears_state](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/close_context_menu_clears_state.md) |
| called_by | [context_menu_action_applies_inner_and_closes_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_action_applies_inner_and_closes_menu.md) |
| called_by | [context_menu_open_submenu_toggles](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_open_submenu_toggles.md) |
| called_by | [show_context_menu_closes_open_active_bar_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_closes_open_active_bar_menu.md) |
| called_by | [show_context_menu_on_all_selection_preserves_all](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_all_selection_preserves_all.md) |
| called_by | [show_context_menu_on_already_selected_graphic_is_idempotent](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_already_selected_graphic_is_idempotent.md) |
| called_by | [show_context_menu_on_empty_leaves_selection_untouched](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_empty_leaves_selection_untouched.md) |
| called_by | [show_context_menu_on_graphic_selects_it_first](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_graphic_selects_it_first.md) |
| called_by | [show_context_menu_on_multiple_member_preserves_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_member_preserves_selection.md) |
| called_by | [show_context_menu_on_multiple_non_member_replaces_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_non_member_replaces_selection.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
