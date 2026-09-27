---
okf_version: "0.2"
type: Function
title: shortcut_chip
resource: crates/oxide-app/src/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/preferences/keymap/shortcut_chip
language: rust
---

# shortcut_chip

## Signature

```rust
fn shortcut_chip(label: &str, tone: ChipTone) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Source
Lines 327–358 in `crates/oxide-app/src/preferences/keymap.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keymap](/crates/oxide-app/src/preferences/keymap.md) |
| called_by | [keymap_recorder_control](/crates/oxide-app/src/preferences/keymap/keymap_recorder_control.md) |
| called_by | [keymap_table_row](/crates/oxide-app/src/preferences/keymap/keymap_table_row.md) |
| called_by | [recorded_shortcut_chips](/crates/oxide-app/src/preferences/keymap/recorded_shortcut_chips.md) |
