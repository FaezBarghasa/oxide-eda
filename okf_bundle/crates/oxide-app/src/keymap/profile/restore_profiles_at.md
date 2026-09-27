---
okf_version: "0.2"
type: Function
title: restore_profiles_at
description: "Move the live shortcuts file aside to a free slot, then write `set`"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/restore_profiles_at
language: rust
---

# restore_profiles_at

Move the live shortcuts file aside to a free slot, then write `set`

## Signature

```rust
pub fn restore_profiles_at(
    path: &Path,
    set: &ShortcutProfileSet,
) -> Result<Option<PathBuf>, ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

Move the live shortcuts file aside to a free slot, then write `set`
in its place. Returns where the previous file went, or `Ok(None)`
when there was no live file to preserve.

The move is what makes a restore safe to offer at any time. The
`.bak` is never cleaned up, so it outlives the failure that produced
it: months later the user may have built a whole new set of profiles
and still have that ancient backup sitting there. Writing straight
over the live file would throw the new work away — the same shape as
the `prefs.json` clobber in #594.

## Source
Lines 555–570 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [free_aside_path](/crates/oxide-app/src/keymap/profile/free_aside_path.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it](/crates/oxide-app/src/keymap/profile_tests/restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it.md) |
