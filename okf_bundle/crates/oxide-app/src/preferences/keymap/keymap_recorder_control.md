---
okf_version: "0.2"
type: Function
title: keymap_recorder_control
resource: crates/oxide-app/src/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/preferences/keymap/keymap_recorder_control
language: rust
---

# keymap_recorder_control

## Signature

```rust
fn keymap_recorder_control(
    recorder: &'a crate::app::KeymapRecorderState,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Source
Lines 360–496 in `crates/oxide-app/src/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/preferences/keymap.md) |
| calls | [row](/crates/oxide-app/src/app/view/dialogs/bom/table/row.md) |
| calls | [recorded_shortcut_chips](/crates/oxide-app/src/preferences/keymap/recorded_shortcut_chips.md) |
| calls | [shortcut_chip](/crates/oxide-app/src/preferences/keymap/shortcut_chip.md) |
| called_by | [content_keyboard_shortcuts](/crates/oxide-app/src/preferences/keymap/content_keyboard_shortcuts.md) |
