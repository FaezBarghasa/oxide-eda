---
okf_version: "0.2"
type: Function
title: insert_custom_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/insert_custom_profile_1
language: rust
---

# insert_custom_profile

## Signature

```rust
pub fn insert_custom_profile(
        &mut self,
        profile: ShortcutProfile,
    ) -> Result<(), ProfileLoadError>
```

## Visibility

- `pub`

## Source
Lines 140–157 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [validate_profile_id](/crates/oxide-app/src/keymap/profile/validate_profile_id.md) |
