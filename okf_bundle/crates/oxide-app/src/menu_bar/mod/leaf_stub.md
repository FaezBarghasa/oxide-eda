---
okf_version: "0.2"
type: Function
title: leaf_stub
description: Leaf menu item — disabled/stub (no action yet).
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/leaf_stub
language: rust
---

# leaf_stub

Leaf menu item — disabled/stub (no action yet).

## Signature

```rust
fn leaf_stub(
    label: &str,
    shortcut: Option<String>,
    mc: MenuColors,
) -> Item<'static, MenuMessage, Theme, iced::Renderer>
```

## Docstring

Leaf menu item — disabled/stub (no action yet).

## Source
Lines 430–436 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| calls | [menu_item_btn](/crates/oxide-app/src/menu_bar/mod/menu_item_btn.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
