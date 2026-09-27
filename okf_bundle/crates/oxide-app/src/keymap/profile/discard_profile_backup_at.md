---
okf_version: "0.2"
type: Function
title: discard_profile_backup_at
description: Delete a profile backup. Only ever called from an explicit user
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/discard_profile_backup_at
language: rust
---

# discard_profile_backup_at

Delete a profile backup. Only ever called from an explicit user

## Signature

```rust
pub fn discard_profile_backup_at(bak: &Path) -> Result<bool, ProfileLoadError>
```

## Visibility

- `pub`

## Docstring

Delete a profile backup. Only ever called from an explicit user
action — a successful save must never remove it, because that is
exactly the moment the user is most likely to want the old profiles
back. `Ok(false)` when there was nothing to delete.

## Source
Lines 576–582 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
| called_by | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
