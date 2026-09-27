---
okf_version: "0.2"
type: Function
title: delete_custom_profile
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/delete_custom_profile
language: rust
---

# delete_custom_profile

## Signature

```rust
impl ShortcutProfileSet { pub fn delete_custom_profile(&mut self, id: &str) -> Result<(), ProfileLoadError> }
```

## Visibility

- `pub`

## Source
Lines 159–172 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
