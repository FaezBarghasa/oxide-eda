---
okf_version: "0.2"
type: Function
title: save_at
description: "Variant for tests / explicit paths — [`prefs_path`] is config-dir-global,"
resource: crates/oxide-app/src/panels/components_panel/global_prefs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/panels/components_panel/global_prefs/save_at
language: rust
---

# save_at

Variant for tests / explicit paths — [`prefs_path`] is config-dir-global,

## Signature

```rust
pub fn save_at(path: &Path, entries: &[GlobalLibraryEntry]) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Variant for tests / explicit paths — [`prefs_path`] is config-dir-global,
so the real `save` is untestable without one. Mirrors the
`save_preferred_order` / `save_preferred_order_at` split in
`library::settings::persistence`.

Crash-safe: [`oxide_types::atomic_io::atomic_write`] writes to a temp
sibling, fsyncs it and renames over the destination, so a crash mid-save
leaves the previous library list intact rather than a truncated file. It
also creates the parent directory, so no separate `create_dir_all` here.

## Source
Lines 114–123 in `crates/oxide-app/src/panels/components_panel/global_prefs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [global_prefs](/crates/oxide-app/src/panels/components_panel/global_prefs.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| called_by | [save](/crates/oxide-app/src/panels/components_panel/global_prefs/save.md) |
| called_by | [save_at_leaves_original_intact_when_write_fails](/crates/oxide-app/src/panels/components_panel/global_prefs/save_at_leaves_original_intact_when_write_fails.md) |
