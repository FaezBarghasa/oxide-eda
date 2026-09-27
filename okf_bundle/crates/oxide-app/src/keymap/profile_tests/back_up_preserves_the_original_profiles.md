---
okf_version: "0.2"
type: Function
title: back_up_preserves_the_original_profiles
description: "The copy aside must be byte-identical, so the custom profiles this"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/back_up_preserves_the_original_profiles
language: rust
---

# back_up_preserves_the_original_profiles

The copy aside must be byte-identical, so the custom profiles this

## Signature

```rust
fn back_up_preserves_the_original_profiles()
```

## Decorators

- `test`

## Docstring

The copy aside must be byte-identical, so the custom profiles this
process could not parse survive the save that follows.
[test]

## Source
Lines 300–314 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
