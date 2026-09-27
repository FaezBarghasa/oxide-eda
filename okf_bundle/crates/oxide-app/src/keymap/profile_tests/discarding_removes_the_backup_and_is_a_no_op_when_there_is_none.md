---
okf_version: "0.2"
type: Function
title: discarding_removes_the_backup_and_is_a_no_op_when_there_is_none
description: Deleting is an explicit action and nothing else may do it — a
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/discarding_removes_the_backup_and_is_a_no_op_when_there_is_none
language: rust
---

# discarding_removes_the_backup_and_is_a_no_op_when_there_is_none

Deleting is an explicit action and nothing else may do it — a

## Signature

```rust
fn discarding_removes_the_backup_and_is_a_no_op_when_there_is_none()
```

## Decorators

- `test`

## Docstring

Deleting is an explicit action and nothing else may do it — a
successful save removing the backup would throw the profiles away at
exactly the moment the user is most likely to want them back.
[test]

## Source
Lines 556–575 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
