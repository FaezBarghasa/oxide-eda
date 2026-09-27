---
okf_version: "0.2"
type: Function
title: init_logging
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/init_logging
language: rust
---

# init_logging

## Signature

```rust
pub fn init_logging() -> Result<()>
```

## Visibility

- `pub`

## Source
Lines 55–62 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [configured_level](/crates/oxide-app/src/diagnostics/configured_level.md) |
| called_by | [asking_whether_a_command_resolves_reports_nothing](/crates/oxide-app/src/app/command/bridge/asking_whether_a_command_resolves_reports_nothing.md) |
| called_by | [dispatching_an_unmapped_command_still_reports_it](/crates/oxide-app/src/app/command/bridge/dispatching_an_unmapped_command_still_reports_it.md) |
| called_by | [a_missing_directory_lists_empty_and_says_nothing](/crates/oxide-app/src/app/dir_listing/a_missing_directory_lists_empty_and_says_nothing.md) |
| called_by | [a_standing_failure_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/dir_listing/a_standing_failure_does_not_flood_the_messages_panel.md) |
| called_by | [an_unreadable_directory_reaches_the_messages_panel](/crates/oxide-app/src/app/dir_listing/an_unreadable_directory_reaches_the_messages_panel.md) |
| called_by | [records_mentioning](/crates/oxide-app/src/app/dir_listing/records_mentioning.md) |
| called_by | [a_post_save_refresh_that_fails_reaches_the_messages_panel](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_reaches_the_messages_panel.md) |
| called_by | [diagnostic_count](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnostic_count.md) |
| called_by | [pdf_export_proceeds_and_warns_once_per_user_action](/crates/oxide-app/src/app/handlers/menu/export/tests/pdf_export_proceeds_and_warns_once_per_user_action.md) |
| called_by | [rerasterizing_the_preview_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/handlers/menu/export/tests/rerasterizing_the_preview_does_not_flood_the_messages_panel.md) |
| called_by | [messages_panel_capture](/crates/oxide-app/src/library/resolve/messages_panel_capture.md) |
| called_by | [a_failed_listing_reaches_the_messages_panel](/crates/oxide-app/src/library/state/tests/a_failed_listing_reaches_the_messages_panel.md) |
| called_by | [main](/crates/oxide-app/src/main/main.md) |
| called_by | [ensure_logger](/crates/oxide-app/tests/regression/diagnostics_routing/ensure_logger.md) |
| called_by | [a_failed_history_walk_reaches_the_messages_panel](/crates/oxide-app/tests/regression/history_load_failure/a_failed_history_walk_reaches_the_messages_panel.md) |
| called_by | [a_refused_cell_commit_reaches_the_messages_panel](/crates/oxide-app/tests/regression/library_browser_cell_commit/a_refused_cell_commit_reaches_the_messages_panel.md) |
