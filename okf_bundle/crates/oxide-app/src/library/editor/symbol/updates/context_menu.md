---
okf_version: "0.2"
type: Module
title: context_menu
description: Symbol editor — right-click context menu update logic. Mirrors
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu
language: rust
---

# context_menu

Symbol editor — right-click context menu update logic. Mirrors

## Docstring

Symbol editor — right-click context menu update logic. Mirrors
`library::editor::footprint::updates::context_menu` in structure.

`SymbolEditorMsg::ContextMenuAction` (apply the boxed action, then
close the menu) is handled directly in `apply_symbol_primitive_edit`
rather than here, since it needs to recurse back into the top-level
dispatcher — every other context-menu variant is state-only and
lives in [`apply_symbol_context_menu`].

## Relationships

| Type | Target |
|------|--------|
| related | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| related | [target_in_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/target_in_selection.md) |
| related | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| related | [show_context_menu_on_empty_leaves_selection_untouched](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_empty_leaves_selection_untouched.md) |
| related | [show_context_menu_on_graphic_selects_it_first](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_graphic_selects_it_first.md) |
| related | [show_context_menu_on_multiple_member_preserves_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_member_preserves_selection.md) |
| related | [show_context_menu_on_multiple_non_member_replaces_selection](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_non_member_replaces_selection.md) |
| related | [show_context_menu_on_all_selection_preserves_all](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_all_selection_preserves_all.md) |
| related | [show_context_menu_on_already_selected_graphic_is_idempotent](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_already_selected_graphic_is_idempotent.md) |
| related | [show_context_menu_closes_open_active_bar_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_closes_open_active_bar_menu.md) |
| related | [close_context_menu_clears_state](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/close_context_menu_clears_state.md) |
| related | [context_menu_open_submenu_toggles](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_open_submenu_toggles.md) |
| related | [context_menu_action_applies_inner_and_closes_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_action_applies_inner_and_closes_menu.md) |
