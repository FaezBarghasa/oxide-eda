---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/library/editor/footprint/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/mod/draw
language: rust
---

# draw

## Signature

```rust
impl FootprintCanvas<'a> { fn draw(
        &self,
        cstate: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> }
```

## Type Parameters

- `'a`

## Source
Lines 276–312 in `crates/oxide-app/src/library/editor/footprint/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/footprint/canvas/mod.md) |
| calls | [draw_pads_tool_preview](/crates/oxide-app/src/library/editor/footprint/canvas/draw/pad/draw_pads_tool_preview.md) |
