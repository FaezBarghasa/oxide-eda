---
okf_version: "0.2"
type: Function
title: check_prefs_file
description: "[`check_prefs_file_at`] against the resolved user prefs path,"
resource: crates/oxide-app/src/fonts/prefs_file.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/prefs_file/check_prefs_file
language: rust
---

# check_prefs_file

[`check_prefs_file_at`] against the resolved user prefs path,

## Signature

```rust
pub fn check_prefs_file() -> Result<(), PrefsLoadError>
```

## Visibility

- `pub`

## Docstring

[`check_prefs_file_at`] against the resolved user prefs path,
mirroring the `x()` / `x_at()` pairing every other writer here uses.

## Source
Lines 227–229 in `crates/oxide-app/src/fonts/prefs_file.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prefs_file](/crates/oxide-app/src/fonts/prefs_file.md) |
| calls | [check_prefs_file_at](/crates/oxide-app/src/fonts/prefs_file/check_prefs_file_at.md) |
| calls | [prefs_file_path](/crates/oxide-app/src/fonts/prefs_file/prefs_file_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
| called_by | [refresh_prefs_load_error](/crates/oxide-app/src/app/handlers/preferences/mod/refresh_prefs_load_error.md) |
