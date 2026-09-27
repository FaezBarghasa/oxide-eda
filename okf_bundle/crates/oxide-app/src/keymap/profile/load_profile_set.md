---
okf_version: "0.2"
type: Function
title: load_profile_set
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/load_profile_set
language: rust
---

# load_profile_set

## Signature

```rust
pub fn load_profile_set() -> Result<ShortcutProfileSet, ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 362–367 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [config_path](/crates/oxide-app/src/keymap/profile/config_path.md) |
| calls | [load_profile_set_at](/crates/oxide-app/src/keymap/profile/load_profile_set_at.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
