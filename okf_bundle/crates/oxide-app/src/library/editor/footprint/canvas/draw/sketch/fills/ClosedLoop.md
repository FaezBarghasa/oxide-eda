---
okf_version: "0.2"
type: Class
title: ClosedLoop
description: "v0.16.1 — Walk the sketch's line graph, find simple closed"
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills/ClosedLoop
language: rust
---

# ClosedLoop

v0.16.1 — Walk the sketch's line graph, find simple closed

## Signature

```rust
pub(in crate::library::editor::footprint::canvas) struct ClosedLoop
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas)`

## Docstring

v0.16.1 — Walk the sketch's line graph, find simple closed
cycles, and render each as a filled polygon. Skips cycles where
every Line is `construction = true` (those are pad-corner
outlines or user-authored guides — already rendered as dashed
strokes elsewhere; double-filling would obscure the rendered
pad). Arc-bounded loops are deferred to v0.16.2.

v0.16.2 — Looks up the role attr on every entity in the loop.
The first hit picks the fill colour from the matching layer in
[`super::super::super::layers::FpLayer`]. Loops with no role assignment fall
back to neutral grey.
v0.27 — closed-loop record exposed to the click handler so a
single click on the polygon fill can select every entity in the
loop. Mirrors what `draw_filled_closed_loops` walks internally.

## Methods

- `lines`
- `points`
- `polygon`

## Source
Lines 27–33 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fills](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/fills.md) |
