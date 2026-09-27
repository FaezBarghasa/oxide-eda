---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/panels/telecom/canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:28:47Z"
concept_id: crates/oxide-app/src/panels/telecom/canvas/draw
language: rust
---

# draw

## Signature

```rust
impl SmithChartCanvas<'a> { fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> }
```

## Type Parameters

- `'a`
- `Message`

## Source
Lines 28–125 in `crates/oxide-app/src/panels/telecom/canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/panels/telecom/canvas.md) |
| calls | [circle](/crates/oxide-app/src/library/editor/footprint/updates/sketch/tools/draw/circle.md) |
