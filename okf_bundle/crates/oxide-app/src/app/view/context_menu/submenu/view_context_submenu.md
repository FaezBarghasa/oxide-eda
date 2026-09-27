---
okf_version: "0.2"
type: Function
title: view_context_submenu
description: Build the secondary submenu (Place / Align / Add New to Project)
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu/view_context_submenu
language: rust
---

# view_context_submenu

Build the secondary submenu (Place / Align / Add New to Project)

## Signature

```rust
impl Oxide { pub(in crate::app::view) fn view_context_submenu(
        &self,
        kind: ContextSubmenu,
    ) -> Element<'_, Message> }
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Build the secondary submenu (Place / Align / Add New to Project)
shown to the right of the parent context menu. Resolves the
selection count / target project from `self`, then delegates to the
pure entry builders and renders via the shared widget.

## Source
Lines 274–299 in `crates/oxide-app/src/app/view/context_menu/submenu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [submenu](/crates/oxide-app/src/app/view/context_menu/submenu.md) |
| calls | [place_entries](/crates/oxide-app/src/app/view/context_menu/submenu/place_entries.md) |
| calls | [align_entries](/crates/oxide-app/src/app/view/context_menu/submenu/align_entries.md) |
| calls | [add_new_entries](/crates/oxide-app/src/app/view/context_menu/submenu/add_new_entries.md) |
