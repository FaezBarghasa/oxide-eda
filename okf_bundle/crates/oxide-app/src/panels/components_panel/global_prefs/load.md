---
okf_version: "0.2"
type: Function
title: load
description: Load the global library list from disk. Returns an empty Vec when
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/load
language: rust
---

# load

Load the global library list from disk. Returns an empty Vec when

## Signature

```rust
pub fn load() -> Vec<GlobalLibraryEntry>
```

## Visibility

- `pub`

## Docstring

Load the global library list from disk. Returns an empty Vec when
the file is missing or unparseable — both non-fatal cases. Parse
errors warn through `tracing` so they're visible without taking the
whole panel down.

## Source
Lines 58–63 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [prefs_path](/crates/oxide-app/src/panels/components_panel/global_prefs/prefs_path.md) |
| calls | [load_at](/crates/oxide-app/src/panels/components_panel/global_prefs/load_at.md) |
| called_by | [is_cancelled](/crates/oxide-app/src/library/settings/digikey_oauth/is_cancelled.md) |
| called_by | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
| called_by | [load_and_mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/load_and_mount_all.md) |
| called_by | [remove_path](/crates/oxide-app/src/panels/components_panel/global_prefs/remove_path.md) |
