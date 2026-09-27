---
okf_version: "0.2"
type: Function
title: log_error
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/log_error
language: rust
---

# log_error

## Signature

```rust
pub fn log_error(context: &str, error: &anyhow::Error)
```

## Visibility

- `pub`

## Source
Lines 76–78 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [dispatch_file_message](/crates/oxide-app/src/app/dispatch/document/dispatch_file_message.md) |
| called_by | [handle_open_primitive](/crates/oxide-app/src/app/dispatch/library/editor/handle_open_primitive.md) |
| called_by | [handle_primitive_editor_event](/crates/oxide-app/src/app/dispatch/library/editor/handle_primitive_editor_event.md) |
| called_by | [handle_mount_finished](/crates/oxide-app/src/app/dispatch/library/lifecycle/handle_mount_finished.md) |
| called_by | [handle_dock_library_browser_message](/crates/oxide-app/src/app/handlers/dock/library_browser/handle_dock_library_browser_message.md) |
| called_by | [load_library_browser_state](/crates/oxide-app/src/app/handlers/dock/library_browser/load_library_browser_state.md) |
| called_by | [handle_add_existing_file_picked](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/handle_add_existing_file_picked.md) |
| called_by | [handle_add_new_schematic_picked](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/handle_add_new_schematic_picked.md) |
| called_by | [register_project_file](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/register_project_file.md) |
| called_by | [handle_app_quit_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_app_quit_confirm.md) |
| called_by | [handle_project_close_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/handle_project_close_confirm.md) |
| called_by | [handle_dock_project_navigation_panel_message](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_dock_project_navigation_panel_message.md) |
| called_by | [handle_project_tree_action](/crates/oxide-app/src/app/handlers/dock/project_navigation/mod/handle_project_tree_action.md) |
| called_by | [handle_remove_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/remove/handle_remove_confirm.md) |
| called_by | [create_new_project](/crates/oxide-app/src/app/handlers/document_files/open/create_new_project.md) |
| called_by | [handle_active_document_save_as_requested](/crates/oxide-app/src/app/handlers/document_files/open/handle_active_document_save_as_requested.md) |
| called_by | [handle_active_document_save_requested](/crates/oxide-app/src/app/handlers/document_files/open/handle_active_document_save_requested.md) |
| called_by | [handle_document_file_opened](/crates/oxide-app/src/app/handlers/document_files/open/handle_document_file_opened.md) |
| called_by | [handle_new_project_file](/crates/oxide-app/src/app/handlers/document_files/open/handle_new_project_file.md) |
| called_by | [open_pcb_file](/crates/oxide-app/src/app/handlers/document_files/open/open_pcb_file.md) |
| called_by | [open_schematic_file](/crates/oxide-app/src/app/handlers/document_files/open/open_schematic_file.md) |
| called_by | [handle_save_primitive_as](/crates/oxide-app/src/app/handlers/document_files/save/handle_save_primitive_as.md) |
| called_by | [save_active_document](/crates/oxide-app/src/app/handlers/document_files/save/save_active_document.md) |
| called_by | [save_active_project_if_dirty](/crates/oxide-app/src/app/handlers/document_files/save/save_active_project_if_dirty.md) |
| called_by | [handle_annotate](/crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate.md) |
| called_by | [handle_reset_duplicate_designators](/crates/oxide-app/src/app/handlers/erc/annotate/handle_reset_duplicate_designators.md) |
| called_by | [apply_engine_command](/crates/oxide-app/src/app/mutation_gateway/apply_engine_command.md) |
| called_by | [apply_engine_commands](/crates/oxide-app/src/app/mutation_gateway/apply_engine_commands.md) |
| called_by | [apply_engine_redo](/crates/oxide-app/src/app/mutation_gateway/apply_engine_redo.md) |
| called_by | [apply_engine_undo](/crates/oxide-app/src/app/mutation_gateway/apply_engine_undo.md) |
| called_by | [resolve_child_reference](/crates/oxide-app/src/app/project_sheets/resolve_child_reference.md) |
| called_by | [a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot](/crates/oxide-app/tests/regression/diagnostics_routing/a_record_emitted_during_a_preferences_message_reaches_the_panel_snapshot.md) |
| called_by | [the_panel_snapshot_is_refreshed_on_every_preferences_message](/crates/oxide-app/tests/regression/diagnostics_routing/the_panel_snapshot_is_refreshed_on_every_preferences_message.md) |
