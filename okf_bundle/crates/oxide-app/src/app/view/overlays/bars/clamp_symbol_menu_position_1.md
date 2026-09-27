---
okf_version: "0.2"
type: Function
title: clamp_symbol_menu_position
description: "Clamp the symbol context menu's requested `(x, y)` so a"
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/clamp_symbol_menu_position_1
language: rust
---

# clamp_symbol_menu_position

Clamp the symbol context menu's requested `(x, y)` so a

## Signature

```rust
fn clamp_symbol_menu_position(
        requested: (f32, f32),
        window: (f32, f32),
        place_submenu_open: bool,
    ) -> (f32, f32)
```

## Docstring

Clamp the symbol context menu's requested `(x, y)` so a
conservative estimate of its footprint stays on screen near
the right / bottom window edges (matches the footprint
overlay's clamping, sized down for the shorter symbol menu).
`place_submenu_open` accounts for the extra rows the Place ▸
submenu adds in place (accordion, not a flyout) when expanded —
otherwise the estimate under-shoots the expanded card's real
footprint and it can run off the bottom edge.

## Source
Lines 522–562 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
