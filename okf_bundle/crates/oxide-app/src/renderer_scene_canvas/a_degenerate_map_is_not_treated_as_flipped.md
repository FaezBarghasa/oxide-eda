---
okf_version: "0.2"
type: Function
title: a_degenerate_map_is_not_treated_as_flipped
description: "A degenerate map must not be read as a reflection — `<` and not `<=`"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/a_degenerate_map_is_not_treated_as_flipped
language: rust
---

# a_degenerate_map_is_not_treated_as_flipped

A degenerate map must not be read as a reflection — `<` and not `<=`

## Signature

```rust
fn a_degenerate_map_is_not_treated_as_flipped()
```

## Decorators

- `test`

## Docstring

A degenerate map must not be read as a reflection — `<` and not `<=`
is what keeps a zero-scale frame on the non-flipping branch, where the
angles pass through untouched.
[test]

## Source
Lines 535–537 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
