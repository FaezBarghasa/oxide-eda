---
okf_version: "0.2"
type: Function
title: collect_track_items_for_library
description: Build the per-row pick-list for the library-scope Enable Version
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/collect_track_items_for_library
language: rust
---

# collect_track_items_for_library

Build the per-row pick-list for the library-scope Enable Version

## Signature

```rust
pub(crate) fn collect_track_items_for_library(
    root_dir: &std::path::Path,
) -> Vec<crate::app::TrackItem>
```

## Visibility

- `pub(crate)`

## Docstring

Build the per-row pick-list for the library-scope Enable Version
Control modal. Each row is a top-level entry inside the library's
`root_dir` — the `library.toml` / `components.tsv` manifest pair
plus any of the canonical subdirectories (`classes/` / `symbols/`
/ `footprints/` / `sims/` / `3dmodels/`) that already exist on
disk. Entries that don't exist are skipped so the picker only
shows real artefacts.

## Source
Lines 313–351 in `crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [version_control](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control.md) |
| called_by | [open_library_enable_version_control_dialog](/crates/oxide-app/src/app/handlers/dock/project_navigation/version_control/open_library_enable_version_control_dialog.md) |
