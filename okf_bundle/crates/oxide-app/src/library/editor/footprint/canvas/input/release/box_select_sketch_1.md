---
okf_version: "0.2"
type: Function
title: box_select_sketch
description: "Sketch-mode rubber-band picker — bbox per entity kind, pick"
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/release/box_select_sketch_1
language: rust
---

# box_select_sketch

Sketch-mode rubber-band picker — bbox per entity kind, pick

## Signature

```rust
fn box_select_sketch(
        &self,
        sketch: &oxide_sketch::SketchData,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

Sketch-mode rubber-band picker — bbox per entity kind, pick
every entity fully inside the rectangle.

## Source
Lines 135–191 in `crates/oxide-app/src/library/editor/footprint/canvas/input/release.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [release](/crates/oxide-app/src/library/editor/footprint/canvas/input/release.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
