---
okf_version: "0.2"
type: Function
title: log_warning
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/log_warning
language: rust
---

# log_warning

## Signature

```rust
pub fn log_warning(message: impl AsRef<str>)
```

## Visibility

- `pub`

## Source
Lines 72–74 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| called_by | [log_unmapped](/crates/oxide-app/src/app/command/bridge/log_unmapped.md) |
| called_by | [append_library_symbols](/crates/oxide-app/src/app/handlers/dock/library_browser/append_library_symbols.md) |
| called_by | [load_library_browser_state](/crates/oxide-app/src/app/handlers/dock/library_browser/load_library_browser_state.md) |
| called_by | [handle_enable_version_control_confirm](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/handle_enable_version_control_confirm.md) |
| called_by | [handle_project_git_commit_done](/crates/oxide-app/src/app/handlers/document_files/git/handle_project_git_commit_done.md) |
| called_by | [handle_history_restore_clicked](/crates/oxide-app/src/app/handlers/document_files/history/handle_history_restore_clicked.md) |
| called_by | [reload_active_tab_from_disk](/crates/oxide-app/src/app/handlers/document_files/history/reload_active_tab_from_disk.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [log_stitch_issues](/crates/oxide-app/src/app/handlers/menu/export/mod/log_stitch_issues.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |
| called_by | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
