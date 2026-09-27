---
okf_version: "0.2"
type: Function
title: refresh_keymap_backup
description: "Re-check whether `keyboard_shortcuts.toml.bak` is on disk (#603)."
resource: crates/oxide-app/src/app/handlers/preferences/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/mod/refresh_keymap_backup_1
language: rust
---

# refresh_keymap_backup

Re-check whether `keyboard_shortcuts.toml.bak` is on disk (#603).

## Signature

```rust
fn refresh_keymap_backup(&mut self)
```

## Docstring

Re-check whether `keyboard_shortcuts.toml.bak` is on disk (#603).

Deliberately not derived from `keymap_load_error`: the backup
outlives the failure that produced it, so the Restore and Discard
controls have to stay reachable long after the banner is gone.

## Source
Lines 101–104 in `crates/oxide-app/src/app/handlers/preferences/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences](/crates/oxide-app/src/app/handlers/preferences/mod.md) |
| calls | [existing_backup_profiles_path](/crates/oxide-app/src/keymap/profile/existing_backup_profiles_path.md) |
