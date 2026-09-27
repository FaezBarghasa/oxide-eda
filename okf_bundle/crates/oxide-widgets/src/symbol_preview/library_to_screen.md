---
okf_version: "0.2"
type: Function
title: library_to_screen
description: "Map a library-space point (Y-up) to frame/screen space (Y-down),"
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/library_to_screen
language: rust
---

# library_to_screen

Map a library-space point (Y-up) to frame/screen space (Y-down),

## Signature

```rust
fn library_to_screen(x: f64, y: f64, mid_x: f64, mid_y: f64, scale: f64, center: Point) -> Point
```

## Docstring

Map a library-space point (Y-up) to frame/screen space (Y-down),
centered and scaled to fit the preview box.

Mirrors the single y-flip in `oxide_types::schematic::SymbolTransform
::apply` / `oxide-output`'s `symbol_world_point`: library Y grows
up, frame Y grows down, so the y term must be negated (about the
bounding-box midpoint) rather than passed through unchanged.

## Source
Lines 43–48 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| called_by | [draw](/crates/oxide-widgets/src/symbol_preview/draw.md) |
| called_by | [library_to_screen_flips_y_about_the_midpoint](/crates/oxide-widgets/src/symbol_preview/library_to_screen_flips_y_about_the_midpoint.md) |
| called_by | [up_pin_screen_tip_is_above_anchor_down_pin_is_below](/crates/oxide-widgets/src/symbol_preview/up_pin_screen_tip_is_above_anchor_down_pin_is_below.md) |
