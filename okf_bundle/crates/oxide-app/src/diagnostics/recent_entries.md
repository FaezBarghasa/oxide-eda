---
okf_version: "0.2"
type: Function
title: recent_entries
resource: crates/oxide-app/src/diagnostics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/diagnostics/recent_entries
language: rust
---

# recent_entries

## Signature

```rust
pub fn recent_entries() -> Vec<DiagnosticEntry>
```

## Visibility

- `pub`

## Source
Lines 80–87 in `crates/oxide-app/src/diagnostics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [diagnostics](/crates/oxide-app/src/diagnostics.md) |
| calls | [entries](/crates/oxide-app/src/diagnostics/entries.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [records_naming](/crates/oxide-app/src/app/command/bridge/records_naming.md) |
| called_by | [records_mentioning](/crates/oxide-app/src/app/dir_listing/records_mentioning.md) |
| called_by | [a_post_save_refresh_that_fails_reaches_the_messages_panel](/crates/oxide-app/src/app/dispatch/library/editor/cache_refresh_tests/a_post_save_refresh_that_fails_reaches_the_messages_panel.md) |
| called_by | [diagnostic_count](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnostic_count.md) |
| called_by | [sync_diagnostics_panel_ctx](/crates/oxide-app/src/app/runtime/mod/sync_diagnostics_panel_ctx.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
| called_by | [entries_mentioning](/crates/oxide-app/src/library/resolve/entries_mentioning.md) |
| called_by | [a_failed_listing_reaches_the_messages_panel](/crates/oxide-app/src/library/state/tests/a_failed_listing_reaches_the_messages_panel.md) |
