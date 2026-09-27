---
okf_version: "0.2"
type: Function
title: classify_entries
description: "Walk the whole ring once and assign `entry` to each"
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/classify_entries
language: rust
---

# classify_entries

Walk the whole ring once and assign `entry` to each

## Signature

```rust
fn classify_entries(verts: &mut [Vertex], start: usize, mut inside: bool)
```

## Docstring

Walk the whole ring once and assign `entry` to each
intersection: alternates from the seed inside-state, true →
false → true → … through the intersections in ring order.

`inside` is the running state — `true` when the current
position is inside the OTHER polygon. We flip it at each
intersection. The intersection's `entry` flag is set to
`inside` before the flip, so an intersection is an "entry"
when the walker is OUTSIDE the other polygon and is about to
step inside.

## Source
Lines 200–210 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
