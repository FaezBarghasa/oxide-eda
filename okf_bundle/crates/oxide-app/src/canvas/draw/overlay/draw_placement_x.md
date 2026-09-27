---
okf_version: "0.2"
type: Function
title: draw_placement_x
description: Unified gray placement crosshair shown for every tool/placement mode.
resource: crates/oxide-app/src/canvas/draw/overlay.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/draw/overlay/draw_placement_x
language: rust
---

# draw_placement_x

Unified gray placement crosshair shown for every tool/placement mode.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn draw_placement_x(
        &self,
        frame: &mut canvas::Frame,
        cursor_pos: iced::Point,
    ) }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Unified gray placement crosshair shown for every tool/placement mode.

## Source
Lines 48–89 in `crates/oxide-app/src/canvas/draw/overlay.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [overlay](/crates/oxide-app/src/canvas/draw/overlay.md) |
