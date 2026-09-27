---
okf_version: "0.2"
type: Function
title: back_up_profile_file_at
description: Copy the existing shortcuts file aside before a save that would
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/back_up_profile_file_at
language: rust
---

# back_up_profile_file_at

Copy the existing shortcuts file aside before a save that would

## Signature

```rust
pub fn back_up_profile_file_at(path: &Path) -> Result<Option<PathBuf>, ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

Copy the existing shortcuts file aside before a save that would
otherwise overwrite profiles this process failed to load (#595).

Three outcomes, all load-bearing:

* `path` does not exist — `Ok(None)`, there is nothing to preserve.
* the `.bak` sibling already exists — `Ok(None)` **without copying**.
That existing backup is the original. A second Apply would
otherwise copy the already-overwritten file over it and destroy the
only surviving copy of the user's profiles.
* otherwise — copy `path` aside and return `Ok(Some(bak_path))`.

An `Err` means the copy could not be made, and the caller must then
refuse to save: overwriting after a failed backup is the data loss
this function exists to prevent, not a fallback.

## Source
Lines 425–437 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [backup_path_for](/crates/oxide-app/src/keymap/profile/backup_path_for.md) |
| called_by | [back_up_profile_file](/crates/oxide-app/src/keymap/profile/back_up_profile_file.md) |
| called_by | [back_up_appends_bak_to_the_whole_file_name](/crates/oxide-app/src/keymap/profile_tests/back_up_appends_bak_to_the_whole_file_name.md) |
| called_by | [back_up_preserves_the_original_profiles](/crates/oxide-app/src/keymap/profile_tests/back_up_preserves_the_original_profiles.md) |
| called_by | [back_up_refuses_to_overwrite_an_existing_backup](/crates/oxide-app/src/keymap/profile_tests/back_up_refuses_to_overwrite_an_existing_backup.md) |
| called_by | [the_users_profiles_survive_an_apply_after_a_failed_load](/crates/oxide-app/src/keymap/profile_tests/the_users_profiles_survive_an_apply_after_a_failed_load.md) |
