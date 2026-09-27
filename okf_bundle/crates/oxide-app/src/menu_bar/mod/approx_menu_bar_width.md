---
okf_version: "0.2"
type: Function
title: approx_menu_bar_width
description: Approximate visible width of the menu bar in pixels. Includes the
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/approx_menu_bar_width
language: rust
---

# approx_menu_bar_width

Approximate visible width of the menu bar in pixels. Includes the

## Signature

```rust
pub fn approx_menu_bar_width() -> f32
```

## Visibility

- `pub`

## Docstring

Approximate visible width of the menu bar in pixels. Includes the
Oxide wordmark on the left, plus the sum of root button widths
(label glyphs at `MENU_LABEL_SIZE` + horizontal padding from
`root_btn`) plus the chrome's left padding. Used by the chrome
to clamp the centered search bar so it can't slide under the
menu items on narrow windows.

## Source
Lines 285–300 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [chrome_search_bar_geometry](/crates/oxide-app/src/app/view/mod/chrome_search_bar_geometry.md) |
| called_by | [search_bar_stays_centred_and_clamped](/crates/oxide-app/src/app/view/mod/search_bar_stays_centred_and_clamped.md) |
