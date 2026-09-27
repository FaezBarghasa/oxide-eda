---
okf_version: "0.2"
type: Function
title: export_custom_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/export_custom_profile
language: rust
---

# export_custom_profile

## Signature

```rust
pub fn export_custom_profile(profile: &ShortcutProfile) -> Result<String, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 592–598 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [exports_and_imports_custom_profile_toml](/crates/oxide-app/src/keymap/profile_tests/exports_and_imports_custom_profile_toml.md) |
