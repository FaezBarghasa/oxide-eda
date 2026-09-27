---
okf_version: "0.2"
type: Function
title: handle_history_restore_clicked
description: "v0.22 Phase 8.5 — Resolve the active tab's path and project,"
resource: crates/oxide-app/src/app/handlers/document_files/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/history/handle_history_restore_clicked
language: rust
---

# handle_history_restore_clicked

v0.22 Phase 8.5 — Resolve the active tab's path and project,

## Signature

```rust
impl Oxide { pub(crate) fn handle_history_restore_clicked(&mut self, sha: &str) }
```

## Visibility

- `pub(crate)`

## Docstring

v0.22 Phase 8.5 — Resolve the active tab's path and project,
then call `LocalGitProjectAdapter::restore_at` with the user-
picked SHA. Marks the file dirty so the next Ctrl+S captures
the restored content.

Best-effort: failures log a warning and surface as a status
message; nothing destructive runs (the working tree changes
only on a successful blob read + atomic write).

## Source
Lines 14–79 in `crates/oxide-app/src/app/handlers/document_files/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/app/handlers/document_files/history.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
