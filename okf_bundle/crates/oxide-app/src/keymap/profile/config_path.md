---
okf_version: "0.2"
type: Function
title: config_path
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/config_path
language: rust
---

# config_path

## Signature

```rust
pub fn config_path() -> Option<PathBuf>
```

## Visibility

- `pub`

## Source
Lines 354–356 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_root](/crates/oxide-app/src/config_root/config_root.md) |
| called_by | [back_up_profile_file](/crates/oxide-app/src/keymap/profile/back_up_profile_file.md) |
| called_by | [backup_profiles_path](/crates/oxide-app/src/keymap/profile/backup_profiles_path.md) |
| called_by | [load_profile_set](/crates/oxide-app/src/keymap/profile/load_profile_set.md) |
| called_by | [save_profile_set](/crates/oxide-app/src/keymap/profile/save_profile_set.md) |
