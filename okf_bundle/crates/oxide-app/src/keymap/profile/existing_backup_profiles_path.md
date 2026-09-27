---
okf_version: "0.2"
type: Function
title: existing_backup_profiles_path
description: "[`backup_profiles_path`] filtered to a backup that is actually on"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/existing_backup_profiles_path
language: rust
---

# existing_backup_profiles_path

[`backup_profiles_path`] filtered to a backup that is actually on

## Signature

```rust
pub fn existing_backup_profiles_path() -> Option<PathBuf>
```

## Visibility

- `pub`

## Docstring

[`backup_profiles_path`] filtered to a backup that is actually on
disk — what the UI asks before offering Restore and Discard.

`Path::exists()` is the right predicate here and only here: it
decides whether to *show* two controls, so treating an un-stat-able
file as absent costs a hidden row, not a lost file. The destructive
paths ([`free_aside_path`], [`discard_profile_backup_at`]) go through
`symlink_metadata` and never through this.

## Source
Lines 466–468 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [backup_profiles_path](/crates/oxide-app/src/keymap/profile/backup_profiles_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [refresh_keymap_backup](/crates/oxide-app/src/app/handlers/preferences/mod/refresh_keymap_backup.md) |
