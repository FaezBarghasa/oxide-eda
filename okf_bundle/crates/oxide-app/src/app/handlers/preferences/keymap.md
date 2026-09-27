---
okf_version: "0.2"
type: Module
title: keymap
description: "Keyboard Shortcuts pane handlers — every `PrefMsg::Keymap*` arm."
resource: crates/oxide-app/src/app/handlers/preferences/keymap.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/preferences/keymap
language: rust
---

# keymap

Keyboard Shortcuts pane handlers — every `PrefMsg::Keymap*` arm.

## Docstring

Keyboard Shortcuts pane handlers — every `PrefMsg::Keymap*` arm.

Split out of the flat `handlers/preferences.rs` when it crossed the
1000-line god-file cap (#603). Pure code motion: every arm body below
is byte-identical to what it was in that file.

The caller routes here by listing all 19 `PrefMsg::Keymap*` variants
explicitly rather than with a `_ =>` catch-all, so its match stays
exhaustive over `PrefMsg`: adding a non-keymap variant is a compile
error there instead of a message silently arriving here.

## Relationships

| Type | Target |
|------|--------|
| related | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| related | [handle_keymap_pref_message](/crates/oxide-app/src/app/handlers/preferences/keymap/handle_keymap_pref_message.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
