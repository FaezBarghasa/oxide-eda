---
okf_version: "0.2"
type: Function
title: a_wrapped_arc_sweeps_the_short_way_in_both_frames
description: "The wrapped case the sweep convention exists for: 330° → 30° is a 60°"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/a_wrapped_arc_sweeps_the_short_way_in_both_frames
language: rust
---

# a_wrapped_arc_sweeps_the_short_way_in_both_frames

The wrapped case the sweep convention exists for: 330° → 30° is a 60°

## Signature

```rust
fn a_wrapped_arc_sweeps_the_short_way_in_both_frames()
```

## Decorators

- `test`

## Docstring

The wrapped case the sweep convention exists for: 330° → 30° is a 60°
arc across zero, not a 300° one. Both handedness values must take the
short way round, in opposite directions.
[test]

## Source
Lines 485–491 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
| calls | [span](/crates/oxide-app/src/renderer_scene_canvas/span.md) |
