---
okf_version: "0.2"
type: Function
title: draw_ghost_label
description: Ghost label / port preview following the cursor.
resource: crates/oxide-app/src/canvas/draw/ghosts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/canvas/draw/ghosts/draw_ghost_label_1
language: rust
---

# draw_ghost_label

Ghost label / port preview following the cursor.

## Signature

```rust
pub(in crate::canvas) fn draw_ghost_label(
        &self,
        frame: &mut canvas::Frame,
        bounds: Rectangle,
        cursor_pos: iced::Point,
    )
```

## Visibility

- `pub(in crate::canvas)`

## Docstring

Ghost label / port preview following the cursor.

## Source
Lines 86–124 in `crates/oxide-app/src/canvas/draw/ghosts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ghosts](/crates/oxide-app/src/canvas/draw/ghosts.md) |
| calls | [draw_label_preview](/crates/oxide-app/src/schematic_runtime/label/draw_label_preview.md) |
