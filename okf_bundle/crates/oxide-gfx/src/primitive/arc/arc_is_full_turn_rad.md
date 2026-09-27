---
okf_version: "0.2"
type: Function
title: arc_is_full_turn_rad
description: "`true` when an arc's raw (unwrapped) `start..end` span is a nonzero"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/arc_is_full_turn_rad
language: rust
---

# arc_is_full_turn_rad

`true` when an arc's raw (unwrapped) `start..end` span is a nonzero

## Signature

```rust
pub fn arc_is_full_turn_rad(start_angle: f32, end_angle: f32) -> bool
```

## Visibility

- `pub`

## Docstring

`true` when an arc's raw (unwrapped) `start..end` span is a nonzero
whole number of full turns: its [`ccw_wrapped_sweep_rad`] collapses
to (near) zero while the raw `end - start` span is genuinely
nonzero. Such an arc is a full circle, not a degenerate zero-sweep
point (`start == end`, where the raw span is also ~0), and must be
drawn and hit-tested as the whole circle outline.

This is the single authority both the CPU canvas draw path
(`renderer_scene_canvas::draw_arc_bucket`) and the symbol body
hit-test (`state::hit_test`'s `Arc` arm) consult, so a full-turn
arc a user typed into the Properties panel (`0° -> 360°`, which
bypasses the load-time full-turn-to-`Circle` migration) never
renders as a visible circle it cannot also click-select.

## Source
Lines 61–65 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
| called_by | [hit_test_graphic_body](/crates/oxide-app/src/library/editor/symbol/state/hit_test/hit_test_graphic_body.md) |
| called_by | [arc_screen_span](/crates/oxide-app/src/renderer_scene_canvas/arc_screen_span.md) |
