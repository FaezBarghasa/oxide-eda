---
okf_version: "0.2"
type: Function
title: view_grid_picker_menu
description: v0.18.10 — Altium-style grid picker popup body. Renders the
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/view_grid_picker_menu_1
language: rust
---

# view_grid_picker_menu

v0.18.10 — Altium-style grid picker popup body. Renders the

## Signature

```rust
pub(in crate::app::view) fn view_grid_picker_menu(&self) -> Element<'_, Message>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

v0.18.10 — Altium-style grid picker popup body. Renders the
standard 1mil…2.5mm ladder; clicking a row sends
`Message::Ui(UiMsg::GridPickerSelect(step_mm))` and closes the popup.

## Source
Lines 36–118 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
