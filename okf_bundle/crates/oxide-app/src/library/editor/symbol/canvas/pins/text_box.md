---
okf_version: "0.2"
type: Function
title: text_box
description: "One label's hit-box: estimate the rendered width from the glyph"
resource: crates/oxide-app/src/library/editor/symbol/canvas/pins.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/pins/text_box
language: rust
---

# text_box

One label's hit-box: estimate the rendered width from the glyph

## Signature

```rust
impl PinRenderGeometry { fn text_box(text: &str, anchor: Vec2d, size_mm: f64, horizontal: bool) -> Aabb }
```

## Docstring

One label's hit-box: estimate the rendered width from the glyph
count, floor it at `size_mm` so a single-char label stays
grabbable, add padding, then centre on `anchor`. `horizontal`
selects which axis the text width runs along.

## Source
Lines 234–263 in `crates/oxide-app/src/library/editor/symbol/canvas/pins.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pins](/crates/oxide-app/src/library/editor/symbol/canvas/pins.md) |
