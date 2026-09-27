---
okf_version: "0.2"
type: Function
title: save_profile_set
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/save_profile_set
language: rust
---

# save_profile_set

## Signature

```rust
pub fn save_profile_set(set: &ShortcutProfileSet) -> Result<(), ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 381–386 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_path](/crates/oxide-app/src/keymap/profile/config_path.md) |
| calls | [save_profile_set_at](/crates/oxide-app/src/keymap/profile/save_profile_set_at.md) |
| called_by | [handle_preferences_message](/crates/oxide-app/src/app/handlers/preferences/mod/handle_preferences_message.md) |
