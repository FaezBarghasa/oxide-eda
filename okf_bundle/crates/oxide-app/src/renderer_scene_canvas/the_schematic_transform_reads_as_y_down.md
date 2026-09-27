---
okf_version: "0.2"
type: Function
title: the_schematic_transform_reads_as_y_down
description: "The schematic's real transform, probed. If someone gives"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/the_schematic_transform_reads_as_y_down
language: rust
---

# the_schematic_transform_reads_as_y_down

The schematic's real transform, probed. If someone gives

## Signature

```rust
fn the_schematic_transform_reads_as_y_down()
```

## Decorators

- `test`

## Docstring

The schematic's real transform, probed. If someone gives
`ScreenTransform` a Y flip, this fails — which is the point: the
handedness is derived from the map, so the two cannot drift apart.
[test]

## Source
Lines 505–517 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
