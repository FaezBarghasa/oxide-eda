---
okf_version: "0.2"
type: Function
title: handle_keymap_pref_message
description: Handle a Keyboard Shortcuts pane message.
resource: crates/oxide-app/src/app/handlers/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message
language: rust
---

# handle_keymap_pref_message

Handle a Keyboard Shortcuts pane message.

## Signature

```rust
impl Oxide { pub(super) fn handle_keymap_pref_message(
        &mut self,
        msg: crate::preferences::PrefMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Handle a Keyboard Shortcuts pane message.

Only reachable for the variants the caller lists, so the
fall-through arm below is dead by construction. It reports rather
than panicking anyway: if that routing list and this match ever
drift apart, a logged line beats taking the UI thread down over a
misrouted message.

## Source
Lines 25–434 in `crates/oxide-app/src/app/handlers/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/app/handlers/preferences/keymap.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [existing_backup_profiles_path](/crates/oxide-app/src/keymap/profile/existing_backup_profiles_path.md) |
| calls | [read_backup_profiles_at](/crates/oxide-app/src/keymap/profile/read_backup_profiles_at.md) |
| calls | [restore_profiles_at](/crates/oxide-app/src/keymap/profile/restore_profiles_at.md) |
| calls | [discard_profile_backup_at](/crates/oxide-app/src/keymap/profile/discard_profile_backup_at.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
| calls | [import_custom_profile](/crates/oxide-app/src/keymap/profile/import_custom_profile.md) |
| calls | [export_custom_profile](/crates/oxide-app/src/keymap/profile/export_custom_profile.md) |
