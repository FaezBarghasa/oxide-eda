---
okf_version: "0.2"
type: Function
title: back_up_appends_bak_to_the_whole_file_name
description: "Guards the `OsString` handling: the backup appends to the whole file"
resource: crates/oxide-app/src/keymap/profile_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile_tests/back_up_appends_bak_to_the_whole_file_name
language: rust
---

# back_up_appends_bak_to_the_whole_file_name

Guards the `OsString` handling: the backup appends to the whole file

## Signature

```rust
fn back_up_appends_bak_to_the_whole_file_name()
```

## Decorators

- `test`

## Docstring

Guards the `OsString` handling: the backup appends to the whole file
name, so the stem AND the `.toml` extension survive. `set_extension`
would silently produce `keyboard_shortcuts.bak` instead.
[test]

## Source
Lines 371–381 in `crates/oxide-app/src/keymap/profile_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile_tests](/crates/oxide-app/src/keymap/profile_tests.md) |
| calls | [seed_saved_profiles](/crates/oxide-app/src/keymap/profile_tests/seed_saved_profiles.md) |
| calls | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
