---
okf_version: "0.2"
type: Function
title: from_profile_set
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/from_profile_set
language: rust
---

# from_profile_set

## Signature

```rust
impl TomlShortcutConfig { fn from_profile_set(set: &ShortcutProfileSet) -> Result<Self, ProfileLoadError> }
```

## Source
Lines 640–653 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
