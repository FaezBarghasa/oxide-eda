---
okf_version: "0.2"
type: Function
title: draw_dashed_line
resource: crates/oxide-app/src/pcb_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/pcb_canvas/draw_dashed_line
language: rust
---

# draw_dashed_line

## Signature

```rust
fn draw_dashed_line(
    frame: &mut canvas::Frame,
    p0: iced::Point,
    p1: iced::Point,
    width: f32,
    color: Color,
)
```

## Source
Lines 278–311 in `crates/oxide-app/src/pcb_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_canvas](/crates/oxide-app/src/pcb_canvas.md) |
| called_by | [draw_lines](/crates/oxide-app/src/pcb_canvas/draw_lines.md) |
