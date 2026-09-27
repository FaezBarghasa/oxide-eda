---
okf_version: "0.2"
type: Function
title: leaf
description: Leaf menu item with an action.
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/leaf
language: rust
---

# leaf

Leaf menu item with an action.

## Signature

```rust
fn leaf(
    label: &str,
    shortcut: Option<String>,
    msg: MenuMessage,
    mc: MenuColors,
) -> Item<'static, MenuMessage, Theme, iced::Renderer>
```

## Docstring

Leaf menu item with an action.

## Source
Lines 420–427 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| calls | [menu_item_btn](/crates/oxide-app/src/menu_bar/mod/menu_item_btn.md) |
| called_by | [view](/crates/oxide-app/src/menu_bar/view/view.md) |
| called_by | [project_root_node](/crates/oxide-app/src/panels/projects/project_root_node.md) |
| called_by | [view_navigator](/crates/oxide-app/src/panels/projects/view_navigator.md) |
