---
okf_version: "0.2"
type: Function
title: a_symbol_editor_arc_reflects_because_its_canvas_flips_y
description: "The Symbol Editor's map *does* flip Y, so there the negation is"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/a_symbol_editor_arc_reflects_because_its_canvas_flips_y
language: rust
---

# a_symbol_editor_arc_reflects_because_its_canvas_flips_y

The Symbol Editor's map *does* flip Y, so there the negation is

## Signature

```rust
fn a_symbol_editor_arc_reflects_because_its_canvas_flips_y()
```

## Decorators

- `test`

## Docstring

The Symbol Editor's map *does* flip Y, so there the negation is
correct and must survive: its screen midpoint is the reflection of the
world one. This is the half that `19a3d4a1` got right.
[test]

## Source
Lines 476–479 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [span](/crates/oxide-app/src/renderer_scene_canvas/span.md) |
