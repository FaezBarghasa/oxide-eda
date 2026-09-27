---
okf_version: "0.2"
type: Function
title: back_up_profile_file
description: "[`back_up_profile_file_at`] against the resolved user config path,"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/back_up_profile_file
language: rust
---

# back_up_profile_file

[`back_up_profile_file_at`] against the resolved user config path,

## Signature

```rust
pub fn back_up_profile_file() -> Result<Option<PathBuf>, ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

[`back_up_profile_file_at`] against the resolved user config path,
mirroring the [`save_profile_set`] / [`save_profile_set_at`] pair.

## Source
Lines 441–446 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_path](/crates/oxide-app/src/keymap/profile/config_path.md) |
| calls | [back_up_profile_file_at](/crates/oxide-app/src/keymap/profile/back_up_profile_file_at.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
