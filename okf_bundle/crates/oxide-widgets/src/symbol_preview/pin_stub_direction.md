---
okf_version: "0.2"
type: Function
title: pin_stub_direction
description: "Direction vector for a pin's stub in **library space** (Y-up): the pin"
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/pin_stub_direction
language: rust
---

# pin_stub_direction

Direction vector for a pin's stub in **library space** (Y-up): the pin

## Signature

```rust
fn pin_stub_direction(rotation: f64) -> (f64, f64)
```

## Docstring

Direction vector for a pin's stub in **library space** (Y-up): the pin
extends from its anchor position toward this unit vector, scaled by
`pin.length`. Rotation is in degrees.

This is the same library-space convention as `oxide-output`'s
`pin_direction` (`crates/oxide-output/src/svg/symbols.rs`) and
`oxide-engine`'s autoplace pass (`transform/autoplace.rs`): 90°
points "up" (`+y`) in Y-up library space. [`library_to_screen`] then
applies the single y-flip that turns that "up" into a smaller
screen-space y, matching every other consumer of library coordinates.

## Source
Lines 26–34 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| called_by | [bounds](/crates/oxide-widgets/src/symbol_preview/bounds.md) |
| called_by | [draw](/crates/oxide-widgets/src/symbol_preview/draw.md) |
| called_by | [up_pin_screen_tip_is_above_anchor_down_pin_is_below](/crates/oxide-widgets/src/symbol_preview/up_pin_screen_tip_is_above_anchor_down_pin_is_below.md) |
