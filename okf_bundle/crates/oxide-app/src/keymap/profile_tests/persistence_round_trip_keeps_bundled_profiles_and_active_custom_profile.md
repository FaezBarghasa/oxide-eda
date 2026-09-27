---
okf_version: "0.2"
type: Function
title: persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile
description: "[test]"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile
language: rust
---

# persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile

[test]

## Signature

```rust
fn persistence_round_trip_keeps_bundled_profiles_and_active_custom_profile()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 100–127 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
| calls | [load_profile_set_at](/crates/oxide-app/src/keymap/profile/load_profile_set_at.md) |
