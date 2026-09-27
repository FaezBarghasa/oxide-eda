---
okf_version: "0.2"
type: Function
title: list_dir_or_report
description: "List the immediate children of `dir`, reporting a real failure."
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/list_dir_or_report
language: rust
---

# list_dir_or_report

List the immediate children of `dir`, reporting a real failure.

## Signature

```rust
pub(in crate::app) fn list_dir_or_report(dir: &Path, context: &str) -> Vec<PathBuf>
```

## Visibility

- `pub(in crate::app)`

## Docstring

List the immediate children of `dir`, reporting a real failure.

`context` names the caller so the record identifies which listing
went missing (`"standard symbol libraries"`, `"library symbols"`, …).
An unreadable directory is reported once per session and then read as
empty, so the caller can carry on; individual unreadable entries are
skipped with a warn.

Entry order is whatever the filesystem hands back — callers that
render a list sort it themselves.

## Source
Lines 28–61 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| calls | [claim_first_report](/crates/oxide-app/src/app/dir_listing/claim_first_report.md) |
| called_by | [a_missing_directory_lists_empty_and_says_nothing](/crates/oxide-app/src/app/dir_listing/a_missing_directory_lists_empty_and_says_nothing.md) |
| called_by | [a_standing_failure_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/dir_listing/a_standing_failure_does_not_flood_the_messages_panel.md) |
| called_by | [an_unreadable_directory_reaches_the_messages_panel](/crates/oxide-app/src/app/dir_listing/an_unreadable_directory_reaches_the_messages_panel.md) |
| called_by | [list_standard_libraries](/crates/oxide-app/src/app/helpers/list_standard_libraries.md) |
| called_by | [build_footprint_editor_panel_ctx](/crates/oxide-app/src/app/runtime/footprint_ctx/build_footprint_editor_panel_ctx.md) |
| called_by | [refresh_panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx/refresh_panel_ctx.md) |
