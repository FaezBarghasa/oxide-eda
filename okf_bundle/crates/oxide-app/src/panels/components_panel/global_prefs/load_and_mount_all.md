---
okf_version: "0.2"
type: Function
title: load_and_mount_all
description: "One-shot \"load and mount\" — used by the bootstrap path so callers"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/load_and_mount_all
language: rust
---

# load_and_mount_all

One-shot "load and mount" — used by the bootstrap path so callers

## Signature

```rust
pub fn load_and_mount_all(
    library_state: &mut crate::library::LibraryState,
) -> Vec<GlobalLibraryEntry>
```

## Visibility

- `pub`

## Docstring

One-shot "load and mount" — used by the bootstrap path so callers
don't have to remember the two-step dance.

## Source
Lines 173–179 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [load](/crates/oxide-app/src/panels/components_panel/global_prefs/load.md) |
| calls | [mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/mount_all.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
