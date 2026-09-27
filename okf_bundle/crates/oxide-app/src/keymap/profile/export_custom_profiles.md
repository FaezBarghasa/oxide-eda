---
okf_version: "0.2"
type: Function
title: export_custom_profiles
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/export_custom_profiles
language: rust
---

# export_custom_profiles

## Signature

```rust
pub fn export_custom_profiles(set: &ShortcutProfileSet) -> Result<String, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 600–603 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
