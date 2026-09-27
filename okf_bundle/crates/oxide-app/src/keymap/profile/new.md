---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/new
language: rust
---

# new

## Signature

```rust
impl ShortcutProfileSet { pub fn new(
        profiles: impl IntoIterator<Item = ShortcutProfile>,
        active_profile_id: impl Into<String>,
    ) -> Result<Self, ProfileLoadError> }
```

## Visibility

- `pub`

## Source
Lines 86–105 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [validate_profile_id](/crates/oxide-app/src/keymap/profile/validate_profile_id.md) |
