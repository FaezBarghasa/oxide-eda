---
okf_version: "0.2"
type: Function
title: up_pin_screen_tip_is_above_anchor_down_pin_is_below
description: "Regression test for issue #495: an Up (90 deg) pin's on-screen tip"
resource: crates/oxide-widgets/src/symbol_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/symbol_preview/up_pin_screen_tip_is_above_anchor_down_pin_is_below
language: rust
---

# up_pin_screen_tip_is_above_anchor_down_pin_is_below

Regression test for issue #495: an Up (90 deg) pin's on-screen tip

## Signature

```rust
fn up_pin_screen_tip_is_above_anchor_down_pin_is_below()
```

## Decorators

- `test`

## Docstring

Regression test for issue #495: an Up (90 deg) pin's on-screen tip
must land above (smaller y) its anchor, and a Down (270 deg) pin's
tip must land below (larger y) its anchor -- not mirrored.

This exercises the exact same pipeline `draw()` uses (`bounds()`
to derive `mid_x`/`mid_y`/`scale`, then `pin_stub_direction` +
`library_to_screen` for the pin endpoints) without needing an
`iced::Renderer`.
[test]

## Source
Lines 404–448 in `crates/oxide-widgets/src/symbol_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_preview](/crates/oxide-widgets/src/symbol_preview.md) |
| calls | [symbol_with_pin](/crates/oxide-widgets/src/symbol_preview/symbol_with_pin.md) |
| calls | [library_to_screen](/crates/oxide-widgets/src/symbol_preview/library_to_screen.md) |
| calls | [pin_stub_direction](/crates/oxide-widgets/src/symbol_preview/pin_stub_direction.md) |
