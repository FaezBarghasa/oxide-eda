---
okf_version: "0.2"
type: Module
title: passive_calculator_modal
description: Tools ▸ Passive Network Calculator modal — the resistor / capacitor /
resource: crates/oxide-app/src/passive_calculator_modal.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/passive_calculator_modal
language: rust
---

# passive_calculator_modal

Tools ▸ Passive Network Calculator modal — the resistor / capacitor /

## Docstring

Tools ▸ Passive Network Calculator modal — the resistor / capacitor /
inductor network calculator plus the RKM encoder, rendered as an
in-app modal like every other Oxide dialog.

It deliberately does NOT open its own OS window. Oxide runs
borderless everywhere (`bootstrap/new.rs` — `decorations: false`),
so a default `iced::window::open` would arrive wearing a native
title bar that exists nowhere else in the app. Same chrome as
`keyboard_shortcuts_modal`: header strip, close X, centred card.

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/passive_calculator_modal/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
