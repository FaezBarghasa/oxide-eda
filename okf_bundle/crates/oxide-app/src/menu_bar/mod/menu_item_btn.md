---
okf_version: "0.2"
type: Function
title: menu_item_btn
description: Build a single menu item button with label + shortcut text.
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/menu_item_btn
language: rust
---

# menu_item_btn

Build a single menu item button with label + shortcut text.

## Signature

```rust
fn menu_item_btn(
    label: &str,
    shortcut: Option<String>,
    msg: Option<MenuMessage>,
    mc: MenuColors,
) -> Element<'static, MenuMessage>
```

## Docstring

Build a single menu item button with label + shortcut text.

## Source
Lines 490–550 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| called_by | [leaf](/crates/oxide-app/src/menu_bar/mod/leaf.md) |
| called_by | [leaf_stub](/crates/oxide-app/src/menu_bar/mod/leaf_stub.md) |
