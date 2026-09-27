---
okf_version: "0.2"
type: Module
title: menu_bar
description: Top menu bar using iced_aw MenuBar with proper dropdown/submenu support.
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod
language: rust
---

# menu_bar

Top menu bar using iced_aw MenuBar with proper dropdown/submenu support.

## Docstring

Top menu bar using iced_aw MenuBar with proper dropdown/submenu support.

Altium-style menu structure: File, Edit, View, Place, Design, Tools, Window, Help.
iced_aw handles all overlay positioning, hover-to-switch, and keyboard navigation.
Anchored on the left by the Oxide wordmark — PNGs rasterised from
`brand/oxide-logo-{white,black}.svg` into `brand/generated/` at 1×/2×/3×
the on-screen 96×31 logical size. Regenerate via
`python installer/build-wordmark.py`.

## Relationships

| Type | Target |
|------|--------|
| related | [wordmark_tier](/crates/oxide-app/src/menu_bar/mod/wordmark_tier.md) |
| related | [MenuMessage](/crates/oxide-app/src/menu_bar/mod/MenuMessage.md) |
| related | [MenuContext](/crates/oxide-app/src/menu_bar/mod/MenuContext.md) |
| related | [default](/crates/oxide-app/src/menu_bar/mod/default.md) |
| related | [default](/crates/oxide-app/src/menu_bar/mod/default.md) |
| related | [approx_menu_bar_width](/crates/oxide-app/src/menu_bar/mod/approx_menu_bar_width.md) |
| related | [MenuColors](/crates/oxide-app/src/menu_bar/mod/MenuColors.md) |
| related | [from_tokens](/crates/oxide-app/src/menu_bar/mod/from_tokens.md) |
| related | [from_tokens](/crates/oxide-app/src/menu_bar/mod/from_tokens.md) |
| related | [cmd_label](/crates/oxide-app/src/menu_bar/mod/cmd_label.md) |
| related | [shortcut_for](/crates/oxide-app/src/menu_bar/mod/shortcut_for.md) |
| related | [wrap_plain](/crates/oxide-app/src/menu_bar/mod/wrap_plain.md) |
| related | [is_dark_surface](/crates/oxide-app/src/menu_bar/mod/is_dark_surface.md) |
| related | [root_btn](/crates/oxide-app/src/menu_bar/mod/root_btn.md) |
| related | [leaf](/crates/oxide-app/src/menu_bar/mod/leaf.md) |
| related | [leaf_stub](/crates/oxide-app/src/menu_bar/mod/leaf_stub.md) |
| related | [submenu_item_btn](/crates/oxide-app/src/menu_bar/mod/submenu_item_btn.md) |
| related | [separator](/crates/oxide-app/src/menu_bar/mod/separator.md) |
| related | [menu_item_btn](/crates/oxide-app/src/menu_bar/mod/menu_item_btn.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
