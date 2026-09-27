---
okf_version: "0.2"
type: Function
title: keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle
description: "Finding 1, keymap half: a pending keyboard-shortcut rebind (which never"
resource: crates/oxide-app/tests/regression/preferences_dirty_guard.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_dirty_guard/keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle
language: rust
---

# keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle

Finding 1, keymap half: a pending keyboard-shortcut rebind (which never

## Signature

```rust
fn keymap_rebind_dirty_flag_survives_an_unrelated_appearance_toggle()
```

## Decorators

- `test`

## Docstring

Finding 1, keymap half: a pending keyboard-shortcut rebind (which never
touches the 7 appearance-draft fields at all) must also keep the dialog
dirty across an appearance toggle.
[test]

## Source
Lines 97–121 in `crates/oxide-app/tests/regression/preferences_dirty_guard.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_dirty_guard](/crates/oxide-app/tests/regression/preferences_dirty_guard.md) |
| calls | [Inner](/crates/oxide-library-server/src/locks/Inner.md) |
