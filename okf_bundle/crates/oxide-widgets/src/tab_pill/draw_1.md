---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-widgets/src/tab_pill.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/tab_pill/draw_1
language: rust
---

# draw

## Signature

```rust
fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    )
```

## Source
Lines 192–273 in `crates/oxide-widgets/src/tab_pill.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tab_pill](/crates/oxide-widgets/src/tab_pill.md) |
| calls | [fill_quad](/crates/oxide-app/src/library/editor/footprint/preview3d/fill_quad.md) |
