---
okf_version: "0.2"
type: Function
title: context_menu_overlay
description: Canvas right-click context menu (+ its Place/Align submenu).
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod/context_menu_overlay
language: rust
---

# context_menu_overlay

Canvas right-click context menu (+ its Place/Align submenu).

## Signature

```rust
impl Oxide { pub(super) fn context_menu_overlay(&self) -> Vec<Element<'_, Message>> }
```

## Visibility

- `pub(super)`

## Docstring

Canvas right-click context menu (+ its Place/Align submenu).
Clamps the menu inside the window; the submenu pops to the right
(or left on overflow) aligned to its launcher row. Pushes dismiss,
menu, then the optional submenu.

## Source
Lines 358–457 in `crates/oxide-app/src/app/view/overlays/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlays](/crates/oxide-app/src/app/view/overlays/mod.md) |
