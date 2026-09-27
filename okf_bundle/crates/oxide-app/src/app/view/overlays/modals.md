---
okf_version: "0.2"
type: Module
title: modals
description: "Dialog and library-modal overlay builders — Preferences, Find &"
resource: crates/oxide-app/src/app/view/overlays/modals.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/modals
language: rust
---

# modals

Dialog and library-modal overlay builders — Preferences, Find &

## Docstring

Dialog and library-modal overlay builders — Preferences, Find &
Replace, keyboard shortcuts, the first-run tour, the simple confirm
dialogs, the detachable annotate/ERC dialogs, and the full stack of
centered-card-on-dim-backdrop library modals, plus the command
palette dropdown and the library-updates modal painted last.

Extracted from the former 1223-line `collect_overlays` god-function
in `view/mod.rs` (behaviour-preserving decomposition). Each builder
owns one overlay's guard + widget tree; ordering is enforced by the
`collect_overlays` assembler.

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_overlay](/crates/oxide-app/src/app/view/overlays/modals/preferences_overlay.md) |
| related | [find_replace_overlay](/crates/oxide-app/src/app/view/overlays/modals/find_replace_overlay.md) |
| related | [keyboard_shortcuts_overlay](/crates/oxide-app/src/app/view/overlays/modals/keyboard_shortcuts_overlay.md) |
| related | [passive_calculator_overlay](/crates/oxide-app/src/app/view/overlays/modals/passive_calculator_overlay.md) |
| related | [first_run_tour_overlay](/crates/oxide-app/src/app/view/overlays/modals/first_run_tour_overlay.md) |
| related | [modal_detached](/crates/oxide-app/src/app/view/overlays/modals/modal_detached.md) |
| related | [rename_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/rename_dialog_overlay.md) |
| related | [remove_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/remove_dialog_overlay.md) |
| related | [project_close_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/project_close_confirm_overlay.md) |
| related | [app_quit_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/app_quit_confirm_overlay.md) |
| related | [project_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/project_options_overlay.md) |
| related | [enable_version_control_overlay](/crates/oxide-app/src/app/view/overlays/modals/enable_version_control_overlay.md) |
| related | [grid_properties_overlay](/crates/oxide-app/src/app/view/overlays/modals/grid_properties_overlay.md) |
| related | [selection_filter_custom_overlay](/crates/oxide-app/src/app/view/overlays/modals/selection_filter_custom_overlay.md) |
| related | [annotate_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/annotate_dialog_overlay.md) |
| related | [annotate_reset_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/annotate_reset_confirm_overlay.md) |
| related | [erc_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/erc_dialog_overlay.md) |
| related | [library_picker_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_picker_overlay.md) |
| related | [new_component_overlay](/crates/oxide-app/src/app/view/overlays/modals/new_component_overlay.md) |
| related | [edit_row_modal_overlay](/crates/oxide-app/src/app/view/overlays/modals/edit_row_modal_overlay.md) |
| related | [delete_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/delete_confirm_overlay.md) |
| related | [primitive_picker_overlay](/crates/oxide-app/src/app/view/overlays/modals/primitive_picker_overlay.md) |
| related | [document_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/document_options_overlay.md) |
| related | [create_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/create_options_overlay.md) |
| related | [close_library_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/close_library_confirm_overlay.md) |
| related | [library_recovery_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_recovery_overlay.md) |
| related | [command_palette_overlay](/crates/oxide-app/src/app/view/overlays/modals/command_palette_overlay.md) |
| related | [library_updates_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_updates_overlay.md) |
| related | [preferences_overlay](/crates/oxide-app/src/app/view/overlays/modals/preferences_overlay.md) |
| related | [find_replace_overlay](/crates/oxide-app/src/app/view/overlays/modals/find_replace_overlay.md) |
| related | [keyboard_shortcuts_overlay](/crates/oxide-app/src/app/view/overlays/modals/keyboard_shortcuts_overlay.md) |
| related | [passive_calculator_overlay](/crates/oxide-app/src/app/view/overlays/modals/passive_calculator_overlay.md) |
| related | [first_run_tour_overlay](/crates/oxide-app/src/app/view/overlays/modals/first_run_tour_overlay.md) |
| related | [modal_detached](/crates/oxide-app/src/app/view/overlays/modals/modal_detached.md) |
| related | [rename_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/rename_dialog_overlay.md) |
| related | [remove_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/remove_dialog_overlay.md) |
| related | [project_close_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/project_close_confirm_overlay.md) |
| related | [app_quit_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/app_quit_confirm_overlay.md) |
| related | [project_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/project_options_overlay.md) |
| related | [enable_version_control_overlay](/crates/oxide-app/src/app/view/overlays/modals/enable_version_control_overlay.md) |
| related | [grid_properties_overlay](/crates/oxide-app/src/app/view/overlays/modals/grid_properties_overlay.md) |
| related | [selection_filter_custom_overlay](/crates/oxide-app/src/app/view/overlays/modals/selection_filter_custom_overlay.md) |
| related | [annotate_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/annotate_dialog_overlay.md) |
| related | [annotate_reset_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/annotate_reset_confirm_overlay.md) |
| related | [erc_dialog_overlay](/crates/oxide-app/src/app/view/overlays/modals/erc_dialog_overlay.md) |
| related | [library_picker_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_picker_overlay.md) |
| related | [new_component_overlay](/crates/oxide-app/src/app/view/overlays/modals/new_component_overlay.md) |
| related | [edit_row_modal_overlay](/crates/oxide-app/src/app/view/overlays/modals/edit_row_modal_overlay.md) |
| related | [delete_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/delete_confirm_overlay.md) |
| related | [primitive_picker_overlay](/crates/oxide-app/src/app/view/overlays/modals/primitive_picker_overlay.md) |
| related | [document_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/document_options_overlay.md) |
| related | [create_options_overlay](/crates/oxide-app/src/app/view/overlays/modals/create_options_overlay.md) |
| related | [close_library_confirm_overlay](/crates/oxide-app/src/app/view/overlays/modals/close_library_confirm_overlay.md) |
| related | [library_recovery_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_recovery_overlay.md) |
| related | [command_palette_overlay](/crates/oxide-app/src/app/view/overlays/modals/command_palette_overlay.md) |
| related | [library_updates_overlay](/crates/oxide-app/src/app/view/overlays/modals/library_updates_overlay.md) |
