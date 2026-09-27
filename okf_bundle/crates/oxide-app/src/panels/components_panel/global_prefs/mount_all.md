---
okf_version: "0.2"
type: Function
title: mount_all
description: "Mount every entry in `entries` onto the supplied `LibraryState`,"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/mount_all
language: rust
---

# mount_all

Mount every entry in `entries` onto the supplied `LibraryState`,

## Signature

```rust
pub fn mount_all(library_state: &mut crate::library::LibraryState, entries: &[GlobalLibraryEntry])
```

## Visibility

- `pub`

## Docstring

Mount every entry in `entries` onto the supplied `LibraryState`,
best-effort. Adapter open failures are logged through `tracing`
and the affected entry is skipped — one bad library shouldn't sink
the whole load.

Called by `bootstrap` once at startup so global libraries are
available across every project the user opens during the session.

## Source
Lines 158–169 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| called_by | [load_and_mount_all](/crates/oxide-app/src/panels/components_panel/global_prefs/load_and_mount_all.md) |
