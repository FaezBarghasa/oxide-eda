---
okf_version: "0.2"
type: Function
title: back_up_refuses_to_overwrite_an_existing_backup
description: "The double-Apply hazard: the second Apply would copy the already"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/back_up_refuses_to_overwrite_an_existing_backup
language: rust
---

# back_up_refuses_to_overwrite_an_existing_backup

The double-Apply hazard: the second Apply would copy the already

## Signature

```rust
fn back_up_refuses_to_overwrite_an_existing_backup()
```

## Decorators

- `test`

## Docstring

The double-Apply hazard: the second Apply would copy the already
overwritten file over the backup and destroy the only surviving copy
of the user's profiles. An existing `.bak` is the original — never
clobber it.
[test]

## Source
Lines 321–351 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
