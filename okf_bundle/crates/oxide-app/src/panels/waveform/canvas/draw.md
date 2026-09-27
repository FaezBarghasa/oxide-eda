---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-app/src/panels/waveform/canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:30:54Z"
concept_id: crates/oxide-app/src/panels/waveform/canvas/draw
language: rust
---

# draw

## Signature

```rust
impl WaveformCanvas<'a> { fn draw(
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
Lines 44–303 in `crates/oxide-app/src/panels/waveform/canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/panels/waveform/canvas.md) |
| calls | [format_axis_value](/crates/oxide-app/src/panels/waveform/canvas/format_axis_value.md) |
