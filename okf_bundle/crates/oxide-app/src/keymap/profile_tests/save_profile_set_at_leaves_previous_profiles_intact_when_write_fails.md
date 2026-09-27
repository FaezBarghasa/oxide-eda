---
okf_version: "0.2"
type: Function
title: save_profile_set_at_leaves_previous_profiles_intact_when_write_fails
description: "`save_profile_set_at` must go through `atomic_write`, not `fs::write`:"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/save_profile_set_at_leaves_previous_profiles_intact_when_write_fails
language: rust
---

# save_profile_set_at_leaves_previous_profiles_intact_when_write_fails

`save_profile_set_at` must go through `atomic_write`, not `fs::write`:

## Signature

```rust
fn save_profile_set_at_leaves_previous_profiles_intact_when_write_fails()
```

## Decorators

- `test`

## Docstring

`save_profile_set_at` must go through `atomic_write`, not `fs::write`:
a failed save leaves the user's previously saved custom keymap profiles
fully intact instead of truncating them.

Discriminator: denying new-file creation in the destination's parent
directory makes `atomic_write`'s `File::create(&tmp)` fail before it can
touch the destination, regardless of the unique per-writer temp name it
picks (#416), and the call returns `Err`. A plain `fs::write` would
ignore that and clobber the old file — so this test fails on a revert.
[test]

## Source
Lines 139–169 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
