---
okf_version: "0.2"
type: Function
title: view_context_menu
description: Canvas right-click menu. Resolves selection state + shortcut hints
resource: crates/oxide-app/src/app/view/context_menu/menus.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/menus/view_context_menu
language: rust
---

# view_context_menu

Canvas right-click menu. Resolves selection state + shortcut hints

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_context_menu(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Canvas right-click menu. Resolves selection state + shortcut hints
from `self`, then delegates to the pure [`canvas_menu_entries`].

## Source
Lines 122–150 in `crates/oxide-app/src/app/view/context_menu/menus.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menus](/crates/oxide-app/src/app/view/context_menu/menus.md) |
| calls | [canvas_menu_entries](/crates/oxide-app/src/app/view/context_menu/menus/canvas_menu_entries.md) |
