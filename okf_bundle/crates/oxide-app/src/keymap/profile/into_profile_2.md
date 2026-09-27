---
okf_version: "0.2"
type: Function
title: into_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/into_profile_2
language: rust
---

# into_profile

## Signature

```rust
impl TomlKeyboardShortcuts { fn into_profile(self) -> Result<ShortcutProfile, ProfileLoadError> }
```

## Source
Lines 752–773 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [validate_profile_id](/crates/oxide-app/src/keymap/profile/validate_profile_id.md) |
