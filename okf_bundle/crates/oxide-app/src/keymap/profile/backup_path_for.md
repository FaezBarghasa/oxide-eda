---
okf_version: "0.2"
type: Function
title: backup_path_for
description: "Sibling `.bak` path — appended to the WHOLE file name, so"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/backup_path_for
language: rust
---

# backup_path_for

Sibling `.bak` path — appended to the WHOLE file name, so

## Signature

```rust
fn backup_path_for(path: &Path) -> Option<PathBuf>
```

## Docstring

Sibling `.bak` path — appended to the WHOLE file name, so
`keyboard_shortcuts.toml` backs up to `keyboard_shortcuts.toml.bak`
rather than the `keyboard_shortcuts.bak` that `set_extension` would
produce. Built through [`std::ffi::OsString`] so a non-UTF-8 config
directory name survives instead of being mangled by a lossy
round-trip. `None` only when `path` has no file name at all.

## Source
Lines 404–408 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
| called_by | [backup_profiles_path](/crates/oxide-app/src/keymap/profile/backup_profiles_path.md) |
