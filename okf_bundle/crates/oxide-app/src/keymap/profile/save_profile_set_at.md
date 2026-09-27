---
okf_version: "0.2"
type: Function
title: save_profile_set_at
description: "Crash-safe: [`oxide_types::atomic_io::atomic_write`] writes to a temp"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/save_profile_set_at
language: rust
---

# save_profile_set_at

Crash-safe: [`oxide_types::atomic_io::atomic_write`] writes to a temp

## Signature

```rust
pub fn save_profile_set_at(path: &Path, set: &ShortcutProfileSet) -> Result<(), ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

Crash-safe: [`oxide_types::atomic_io::atomic_write`] writes to a temp
sibling, fsyncs it and renames over the destination, so a crash mid-save
leaves the user's previously saved keymap profiles intact rather than a
truncated file. It also creates the parent directory, so no separate
`create_dir_all` here.

## Source
Lines 393–396 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [export_custom_profiles](/crates/oxide-app/src/keymap/profile/export_custom_profiles.md) |
| calls | [atomic_write](/crates/oxide-app/src/app/dispatch/library/recovery/atomic_write.md) |
| called_by | [restore_profiles_at](/crates/oxide-app/src/keymap/profile/restore_profiles_at.md) |
| called_by | [save_profile_set](/crates/oxide-app/src/keymap/profile/save_profile_set.md) |
| called_by | [persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile](/crates/oxide-app/src/keymap/profile_tests/persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile.md) |
| called_by | [save_profile_set_at_leaves_previous_profiles_intact_when_write_fails](/crates/oxide-app/src/keymap/profile_tests/save_profile_set_at_leaves_previous_profiles_intact_when_write_fails.md) |
| called_by | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| called_by | [the_users_profiles_survive_an_apply_after_a_failed_load](/crates/oxide-app/src/keymap/profile_tests/the_users_profiles_survive_an_apply_after_a_failed_load.md) |
