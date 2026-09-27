---
okf_version: "0.2"
type: Function
title: backup_profiles_path
description: "The `.bak` sibling of the resolved shortcuts file, whether or not it"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/backup_profiles_path
language: rust
---

# backup_profiles_path

The `.bak` sibling of the resolved shortcuts file, whether or not it

## Signature

```rust
pub fn backup_profiles_path() -> Option<PathBuf>
```

## Visibility

- `pub`

## Docstring

The `.bak` sibling of the resolved shortcuts file, whether or not it
exists. `None` only when there is no config directory at all.

Exposed so the Preferences pane can say the backup is there after the
load-error banner has gone (#603) — before this, the only mention of
the file was a transient status line at the moment it was written.

## Source
Lines 454–456 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_path](/crates/oxide-app/src/keymap/profile/config_path.md) |
| calls | [backup_path_for](/crates/oxide-app/src/keymap/profile/backup_path_for.md) |
| called_by | [existing_backup_profiles_path](/crates/oxide-app/src/keymap/profile/existing_backup_profiles_path.md) |
