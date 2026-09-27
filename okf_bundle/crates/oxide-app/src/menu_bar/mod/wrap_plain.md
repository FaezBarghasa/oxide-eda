---
okf_version: "0.2"
type: Function
title: wrap_plain
description: Wrap a menu element in the toolbar-strip styled container used on
resource: crates/oxide-app/src/menu_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/menu_bar/mod/wrap_plain
language: rust
---

# wrap_plain

Wrap a menu element in the toolbar-strip styled container used on

## Signature

```rust
pub fn wrap_plain(menu: Element<'a, M>, tokens: &ThemeTokens) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Wrap a menu element in the toolbar-strip styled container used on
secondary (undocked-tab) windows that keep their OS title bar.

## Source
Lines 359–365 in `crates/oxide-app/src/menu_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [menu_bar](/crates/oxide-app/src/menu_bar/mod.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
| called_by | [view_main_for](/crates/oxide-app/src/app/view/mod/view_main_for.md) |
