---
okf_version: "0.2"
type: Function
title: scan_library_updates_for_open_schematic
description: "Scan the schematic at `path` for placed Symbols whose"
resource: crates/oxide-app/src/app/dispatch/library/updates.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/updates/scan_library_updates_for_open_schematic
language: rust
---

# scan_library_updates_for_open_schematic

Scan the schematic at `path` for placed Symbols whose

## Signature

```rust
impl Oxide { pub(crate) fn scan_library_updates_for_open_schematic(
        &mut self,
        schematic_path: std::path::PathBuf,
    ) }
```

## Visibility

- `pub(crate)`

## Docstring

Scan the schematic at `path` for placed Symbols whose
`library_version` drifted from the source row's current
version. Splits into two control paths:

* **Personal** workflow mode — auto-applies every drift to the
schematic engine silently and dirty-marks the path.
* **Team** workflow mode — populates
`LibraryState::library_updates` with the entries so
`view_main_for` opens the modal on the next tick.

Adapter / library-mount failures degrade to a single
`tracing::warn` line per affected library and skip those
entries — a missing library doesn't abort the schematic open.
Symbols without a `library_id` (Standard-imported, hand-built)
are skipped silently.

## Source
Lines 52–215 in `crates/oxide-app/src/app/dispatch/library/updates.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/app/dispatch/library/updates.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [classify_bump](/crates/oxide-app/src/library/updates_dialog/classify_bump.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
