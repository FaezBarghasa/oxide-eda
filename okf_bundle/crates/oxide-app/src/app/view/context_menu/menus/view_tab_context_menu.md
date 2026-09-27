---
okf_version: "0.2"
type: Function
title: view_tab_context_menu
description: Build the document-tab right-click menu. Resolves the tab title and
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/view_tab_context_menu
language: rust
---

# view_tab_context_menu

Build the document-tab right-click menu. Resolves the tab title and

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_tab_context_menu(
        &self,
        ctx: &crate::app::TabContextMenuState,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Build the document-tab right-click menu. Resolves the tab title and
undock state from `self`, then delegates to [`tab_menu_entries`].

## Source
Lines 154–177 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
| calls | [tab_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/tab_menu_entries.md) |
