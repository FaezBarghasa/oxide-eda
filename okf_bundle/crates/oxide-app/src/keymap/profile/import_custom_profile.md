---
okf_version: "0.2"
type: Function
title: import_custom_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/import_custom_profile
language: rust
---

# import_custom_profile

## Signature

```rust
pub fn import_custom_profile(source: &str) -> Result<ShortcutProfile, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 584–590 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| called_by | [exports_and_imports_custom_profile_toml](/crates/oxide-app/src/keymap/profile_tests/exports_and_imports_custom_profile_toml.md) |
| called_by | [import_rejects_built_in_profile_documents](/crates/oxide-app/src/keymap/profile_tests/import_rejects_built_in_profile_documents.md) |
