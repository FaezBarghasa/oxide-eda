---
okf_version: "0.2"
type: Function
title: update_right_pressed
description: "Right-press: cancel drag, else start pan or Active Bar dropdown."
resource: crates/oxide-app/src/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/canvas/input/pointer/update_right_pressed
language: rust
---

# update_right_pressed

Right-press: cancel drag, else start pan or Active Bar dropdown.

## Signature

```rust
impl SchematicCanvas<'_> { pub(in crate::canvas) fn update_right_pressed(
        &self,
        state: &mut CanvasState,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> }
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Right-press: cancel drag, else start pan or Active Bar dropdown.

## Source
Lines 185–222 in `crates/oxide-app/src/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/canvas/input/pointer.md) |
| calls | [active_bar_hit](/crates/oxide-app/src/canvas/mod/active_bar_hit.md) |
