---
okf_version: "0.2"
type: Function
title: reload_active_tab_from_disk
description: "v0.22 — Re-parse the active tab's on-disk file and replace"
resource: crates/oxide-app/src/app/handlers/document_files/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/document_files/history/reload_active_tab_from_disk_1
language: rust
---

# reload_active_tab_from_disk

v0.22 — Re-parse the active tab's on-disk file and replace

## Signature

```rust
pub(crate) fn reload_active_tab_from_disk(&mut self)
```

## Visibility

- `pub(crate)`

## Docstring

v0.22 — Re-parse the active tab's on-disk file and replace
the in-memory engine/editor state with the fresh content.
Used after a `restore_at` to make the rewind visible without
requiring the user to close and reopen the tab.

Per-tab-kind dispatch:
- **Schematic**: re-parse `.snxsch` via `SnxSchematic::parse`,
replace the engine via `sync_engine_from_schematic`, refresh
the canvas.
- **PCB**: re-parse `.snxpcb` via `SnxPcb::parse`, replace
the tab's cached_document, refresh the renderer snapshot.
- **FootprintEditor**: re-parse the `.snxfpt` JSON, replace
the entry in `document_state.footprint_editors`, clear the
canvas cache.
- **SymbolEditor**: re-parse the `.snxsym` JSON, replace the
entry in `document_state.symbol_editors`, clear the canvas
cache.
- **LibraryBrowser / ComponentEditor**: deferred — these tabs
read from the library adapter which has its own refresh
path; restore-to-historical-version on these would need a
library-side reload.

## Source
Lines 102–284 in `crates/oxide-app/src/app/handlers/document_files/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/app/handlers/document_files/history.md) |
| calls | [log_warning](/crates/oxide-app/src/diagnostics/log_warning.md) |
| calls | [log_info](/crates/oxide-app/src/diagnostics/log_info.md) |
