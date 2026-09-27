---
okf_version: "0.2"
type: Class
title: SymbolCanvas
description: "Builder for the per-render [`SymbolCanvas`] — all the inputs the"
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/SymbolCanvas
language: rust
---

# SymbolCanvas

Builder for the per-render [`SymbolCanvas`] — all the inputs the

## Signature

```rust
pub struct SymbolCanvas
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Builder for the per-render [`SymbolCanvas`] — all the inputs the
canvas needs from the surrounding state. The canvas itself is
constructed fresh on every iced view tick (see
`library/editor/standalone.rs::view_symbol_canvas`).

## Methods

- `symbol`
- `selected`
- `tool`
- `polygon_vertices`
- `active_part`
- `context_menu_open`
- `camera`
- `grid_size_mm`
- `grid_visible`
- `grid_style`
- `pin_label_grab`
- `bg_color`
- `grid_color`
- `body_color`
- `pin_color`
- `selected_color`
- `text_color`
- `axis_color`

## Source
Lines 67–123 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
