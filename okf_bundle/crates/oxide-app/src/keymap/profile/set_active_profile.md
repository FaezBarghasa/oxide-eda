---
okf_version: "0.2"
type: Function
title: set_active_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/set_active_profile
language: rust
---

# set_active_profile

## Signature

```rust
impl ShortcutProfileSet { pub fn set_active_profile(&mut self, id: impl Into<String>) -> Result<(), ProfileLoadError> }
```

## Visibility

- `pub`

## Source
Lines 131–138 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
