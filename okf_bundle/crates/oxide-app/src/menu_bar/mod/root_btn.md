---
okf_version: "0.2"
type: Function
title: root_btn
description: Root-level menu button (top bar).
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/root_btn
language: rust
---

# root_btn

Root-level menu button (top bar).

## Signature

```rust
fn root_btn(label: &str, mc: MenuColors) -> Element<'static, MenuMessage>
```

## Docstring

Root-level menu button (top bar).

Altium paints a subtle framed highlight behind the label on hover and
keeps it lit while the dropdown is open. `button::Status::Hovered` covers
the pointer case; `Pressed` is the "menu is open" state (iced_aw holds
the root in Pressed while its submenu is visible).

## Source
Lines 386–417 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
