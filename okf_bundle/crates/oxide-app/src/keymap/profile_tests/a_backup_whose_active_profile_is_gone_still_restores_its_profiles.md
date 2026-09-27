---
okf_version: "0.2"
type: Function
title: a_backup_whose_active_profile_is_gone_still_restores_its_profiles
description: The most likely reason the file failed to load is a dangling
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/a_backup_whose_active_profile_is_gone_still_restores_its_profiles
language: rust
---

# a_backup_whose_active_profile_is_gone_still_restores_its_profiles

The most likely reason the file failed to load is a dangling

## Signature

```rust
fn a_backup_whose_active_profile_is_gone_still_restores_its_profiles()
```

## Decorators

- `test`

## Docstring

The most likely reason the file failed to load is a dangling
`active_profile`, and it is fully recoverable: the profiles are all
there, only the pointer is stale. Refusing the whole restore over it
would strand the user's work in a file the app can read perfectly.
[test]

## Source
Lines 453–482 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [read_backup_profiles_at](/crates/oxide-app/src/keymap/profile/read_backup_profiles_at.md) |
