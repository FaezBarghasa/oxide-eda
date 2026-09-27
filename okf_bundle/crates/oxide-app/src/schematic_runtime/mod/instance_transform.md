---
okf_version: "0.2"
type: Function
title: instance_transform
description: "[inline]"
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/instance_transform
language: rust
---

# instance_transform

[inline]

## Signature

```rust
pub fn instance_transform(symbol: &Symbol, local_point: &Point) -> (f64, f64)
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

[inline]

## Source
Lines 151–166 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [draw_move_guides](/crates/oxide-app/src/canvas/draw/drag/draw_move_guides.md) |
| called_by | [text_prop_aabb](/crates/oxide-app/src/schematic_runtime/mod/text_prop_aabb.md) |
