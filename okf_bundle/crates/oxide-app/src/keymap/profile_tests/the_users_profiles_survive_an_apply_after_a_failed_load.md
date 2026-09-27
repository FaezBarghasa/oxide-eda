---
okf_version: "0.2"
type: Function
title: the_users_profiles_survive_an_apply_after_a_failed_load
description: "The whole of #595 end to end, in the order the user hits it: a"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/the_users_profiles_survive_an_apply_after_a_failed_load
language: rust
---

# the_users_profiles_survive_an_apply_after_a_failed_load

The whole of #595 end to end, in the order the user hits it: a

## Signature

```rust
fn the_users_profiles_survive_an_apply_after_a_failed_load()
```

## Decorators

- `test`

## Docstring

The whole of #595 end to end, in the order the user hits it: a
recoverable file rots, the load fails, the app boots on the built-ins,
and the user opens Preferences and presses Apply to rebuild what
vanished. That Apply serialises only `Custom` profiles and the fallback
has none, so the write itself is unavoidably profile-free — the
property that has to hold is that their profiles are still recoverable
afterwards, and that pressing Apply a second time does not take that
away too.
[test]

## Source
Lines 392–443 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
