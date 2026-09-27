---
okf_version: "0.2"
type: Function
title: restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it
description: "The `.bak` is never cleaned up, so it outlives the failure that made"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it
language: rust
---

# restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it

The `.bak` is never cleaned up, so it outlives the failure that made

## Signature

```rust
fn restoring_moves_the_current_shortcuts_file_aside_instead_of_overwriting_it()
```

## Decorators

- `test`

## Docstring

The `.bak` is never cleaned up, so it outlives the failure that made
it: months later the user may have a whole new set of profiles. A
restore that wrote straight over them would be the `prefs.json`
clobber of #594 with extra steps.
[test]

## Source
Lines 517–550 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [read_backup_profiles_at](/crates/oxide-app/src/keymap/profile/read_backup_profiles_at.md) |
| calls | [restore_profiles_at](/crates/oxide-app/src/keymap/profile/restore_profiles_at.md) |
