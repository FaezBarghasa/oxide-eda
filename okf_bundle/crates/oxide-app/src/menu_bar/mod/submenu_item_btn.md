---
okf_version: "0.2"
type: Function
title: submenu_item_btn
description: "Menu row that acts as a submenu header — label on the left, right"
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/submenu_item_btn
language: rust
---

# submenu_item_btn

Menu row that acts as a submenu header — label on the left, right

## Signature

```rust
fn submenu_item_btn(label: &str, mc: MenuColors) -> Element<'static, MenuMessage>
```

## Docstring

Menu row that acts as a submenu header — label on the left, right
chevron on the right, no shortcut. Does not dispatch on click; the
menu framework opens the nested submenu on hover.

## Source
Lines 441–473 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
