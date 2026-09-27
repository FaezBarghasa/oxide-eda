---
okf_version: "0.2"
type: Function
title: log_info
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/log_info
language: rust
---

# log_info

## Signature

```rust
pub fn log_info(message: impl AsRef<str>)
```

## Visibility

- `pub`

## Source
Lines 68–70 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [dispatch_update](/crates/oxide-app/src/app/dispatch/mod/dispatch_update.md) |
| called_by | [handle_active_bar_action](/crates/oxide-app/src/app/handlers/active_bar/action_groups/handle_active_bar_action.md) |
| called_by | [handle_active_bar_placement_preset](/crates/oxide-app/src/app/handlers/active_bar/placement_presets/handle_active_bar_placement_preset.md) |
| called_by | [open_or_focus_child_sheet](/crates/oxide-app/src/app/handlers/canvas/mod/open_or_focus_child_sheet.md) |
| called_by | [handle_enable_version_control_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm.md) |
| called_by | [handle_project_git_commit_done](/crates/oxide-app/src/app/handlers/document_files/git/handle_project_git_commit_done.md) |
| called_by | [handle_history_restore_clicked](/crates/oxide-app/src/app/handlers/document_files/history/handle_history_restore_clicked.md) |
| called_by | [reload_active_tab_from_disk](/crates/oxide-app/src/app/handlers/document_files/history/reload_active_tab_from_disk.md) |
| called_by | [handle_save_primitive_as](/crates/oxide-app/src/app/handlers/document_files/save/handle_save_primitive_as.md) |
| called_by | [persist_project_by_id](/crates/oxide-app/src/app/handlers/document_files/save/persist_project_by_id.md) |
| called_by | [save_active_document](/crates/oxide-app/src/app/handlers/document_files/save/save_active_document.md) |
| called_by | [save_active_document_as](/crates/oxide-app/src/app/handlers/document_files/save/save_active_document_as.md) |
| called_by | [handle_annotate](/crates/oxide-app/src/app/handlers/erc/annotate/handle_annotate.md) |
| called_by | [handle_reset_duplicate_designators](/crates/oxide-app/src/app/handlers/erc/annotate/handle_reset_duplicate_designators.md) |
| called_by | [ensure_sheet_open_and_active](/crates/oxide-app/src/app/handlers/erc/erc_run/ensure_sheet_open_and_active.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [load_project_dsl_eval_fns](/crates/oxide-app/src/app/handlers/erc/erc_run/load_project_dsl_eval_fns.md) |
| called_by | [handle_menu_editing_command](/crates/oxide-app/src/app/handlers/menu/editing/handle_menu_editing_command.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |
| called_by | [active_bar_stub](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_stub.md) |
| called_by | [apply_symbol_ui](/crates/oxide-app/src/library/editor/symbol/updates/ui/apply_symbol_ui.md) |
