---
okf_version: "0.2"
type: Function
title: an_unparseable_backup_fails_the_restore_and_changes_nothing
description: "An unparseable backup is not recoverable, and the restore has to say"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/an_unparseable_backup_fails_the_restore_and_changes_nothing
language: rust
---

# an_unparseable_backup_fails_the_restore_and_changes_nothing

An unparseable backup is not recoverable, and the restore has to say

## Signature

```rust
fn an_unparseable_backup_fails_the_restore_and_changes_nothing()
```

## Decorators

- `test`

## Docstring

An unparseable backup is not recoverable, and the restore has to say
so rather than half-importing or silently producing built-ins.
[test]

## Source
Lines 487–510 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [read_backup_profiles_at](/crates/oxide-app/src/keymap/profile/read_backup_profiles_at.md) |
