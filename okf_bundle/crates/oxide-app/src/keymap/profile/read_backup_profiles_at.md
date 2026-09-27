---
okf_version: "0.2"
type: Function
title: read_backup_profiles_at
description: "Read the profiles out of a backup file, through the normal loader."
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/read_backup_profiles_at
language: rust
---

# read_backup_profiles_at

Read the profiles out of a backup file, through the normal loader.

## Signature

```rust
pub fn read_backup_profiles_at(bak: &Path) -> Result<RestoredProfiles, ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

Read the profiles out of a backup file, through the normal loader.

Deliberately NOT "copy the `.bak` back over the live file". The
backup holds the file that failed to load; putting it back would
reproduce the same failure on the next launch. Parsing it instead
means an unrepairable backup fails here, cleanly, with both files
untouched — and a backup whose only fault was a dangling
`active_profile` is recovered in full.

## Source
Lines 535–543 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [a_backup_whose_active_profile_is_gone_still_restores_its_profiles](/crates/oxide-app/src/keymap/profile_tests/a_backup_whose_active_profile_is_gone_still_restores_its_profiles.md) |
| called_by | [an_unparseable_backup_fails_the_restore_and_changes_nothing](/crates/oxide-app/src/keymap/profile_tests/an_unparseable_backup_fails_the_restore_and_changes_nothing.md) |
| called_by | [restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it](/crates/oxide-app/src/keymap/profile_tests/restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it.md) |
