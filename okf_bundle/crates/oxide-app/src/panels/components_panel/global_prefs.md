---
okf_version: "0.2"
type: Module
title: global_prefs
description: Oxide-wide global library preferences.
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs
language: rust
---

# global_prefs

Oxide-wide global library preferences.

## Docstring

Oxide-wide global library preferences.

Stage 9 of `v0.9-snxlib-as-file-plan.md`. The Components Panel
surfaces three mount sources: Project (auto-mounted from
`Project.libraries`), Installed (session-scoped, in-memory), and
Global. This module owns the on-disk persistence for the Global
source — a single TOML file at
`<config_dir>/oxide/global_libraries.toml`.

Schema (TOML):
```toml
[[libraries]]
path = "C:\\Users\\caner\\Documents\\Libraries\\Common.snxlib"
remote = "git@github.com:caner/common-lib.git"   # optional
auto_pull = true                                  # optional
```

All fields except `path` are optional. The whole file is optional —
a missing or unparseable file warns through `tracing` and the
Components Panel renders the Global section empty (no panic).

## Relationships

| Type | Target |
|------|--------|
| related | [GlobalLibraryEntry](/crates/oxide-app/src/panels/components_panel/global_prefs/GlobalLibraryEntry.md) |
| related | [GlobalPrefsFile](/crates/oxide-app/src/panels/components_panel/global_prefs/GlobalPrefsFile.md) |
| related | [prefs_path](/crates/oxide-app/src/panels/components_panel/global_prefs/prefs_path.md) |
| related | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |
| related | [load_at](/crates/oxide-app/src/panels/components_panel/global_prefs/load_at.md) |
| related | [save](/crates/oxide-app/src/panels/components_panel/global_prefs/save.md) |
| related | [save_at](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at.md) |
| related | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| related | [remove_path](/crates/oxide-app/src/panels/components_panel/global_prefs/remove_path.md) |
| related | [mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/mount_all.md) |
| related | [load_and_mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/load_and_mount_all.md) |
| related | [load_returns_empty_when_file_missing](/crates/oxide-app/src/panels/components_panel/global_prefs/load_returns_empty_when_file_missing.md) |
| related | [round_trip_serialises_path_only_entry](/crates/oxide-app/src/panels/components_panel/global_prefs/round_trip_serialises_path_only_entry.md) |
| related | [round_trip_preserves_remote_and_auto_pull](/crates/oxide-app/src/panels/components_panel/global_prefs/round_trip_preserves_remote_and_auto_pull.md) |
| related | [save_at_leaves_original_intact_when_write_fails](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at_leaves_original_intact_when_write_fails.md) |
| related | [malformed_toml_returns_empty_via_load_path_logic](/crates/oxide-app/src/panels/components_panel/global_prefs/malformed_toml_returns_empty_via_load_path_logic.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
