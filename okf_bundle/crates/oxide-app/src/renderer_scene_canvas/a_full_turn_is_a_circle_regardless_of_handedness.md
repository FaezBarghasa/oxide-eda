---
okf_version: "0.2"
type: Function
title: a_full_turn_is_a_circle_regardless_of_handedness
description: A whole-turn span is a circle on both surfaces — its wrapped sweep
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/a_full_turn_is_a_circle_regardless_of_handedness
language: rust
---

# a_full_turn_is_a_circle_regardless_of_handedness

A whole-turn span is a circle on both surfaces — its wrapped sweep

## Signature

```rust
fn a_full_turn_is_a_circle_regardless_of_handedness()
```

## Decorators

- `test`

## Docstring

A whole-turn span is a circle on both surfaces — its wrapped sweep
collapses to zero, which would otherwise draw nothing at all.
[test]

## Source
Lines 496–499 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
