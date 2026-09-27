---
okf_version: "0.2"
type: Function
title: recorded_shortcut_chips
resource: crates/oxide-app/src/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/preferences/keymap/recorded_shortcut_chips
language: rust
---

# recorded_shortcut_chips

## Signature

```rust
fn recorded_shortcut_chips(
    recorder: &'a crate::app::KeymapRecorderState,
) -> Vec<Element<'a, PrefMsg>>
```

## Type Parameters

- `'a`

## Source
Lines 498–506 in `crates/oxide-app/src/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/preferences/keymap.md) |
| calls | [shortcut_chip](/crates/oxide-app/src/preferences/keymap/shortcut_chip.md) |
| called_by | [keymap_recorder_control](/crates/oxide-app/src/preferences/keymap/keymap_recorder_control.md) |
