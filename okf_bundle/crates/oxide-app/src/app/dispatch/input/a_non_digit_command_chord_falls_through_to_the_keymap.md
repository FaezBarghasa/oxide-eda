---
okf_version: "0.2"
type: Function
title: a_non_digit_command_chord_falls_through_to_the_keymap
description: "#127 — without the `selection_slot_from_key` guard these arms"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/a_non_digit_command_chord_falls_through_to_the_keymap
language: rust
---

# a_non_digit_command_chord_falls_through_to_the_keymap

#127 — without the `selection_slot_from_key` guard these arms

## Signature

```rust
fn a_non_digit_command_chord_falls_through_to_the_keymap()
```

## Decorators

- `test`

## Docstring

#127 — without the `selection_slot_from_key` guard these arms
matched EVERY Ctrl/Alt chord and swallowed it, shadowing
Ctrl+C/X/V/D before they reached the keymap resolver.
[test]

## Source
Lines 772–783 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| calls | [quiet_app](/crates/oxide-app/src/app/dispatch/input/quiet_app.md) |
| calls | [main_window](/crates/oxide-app/src/app/dispatch/input/main_window.md) |
