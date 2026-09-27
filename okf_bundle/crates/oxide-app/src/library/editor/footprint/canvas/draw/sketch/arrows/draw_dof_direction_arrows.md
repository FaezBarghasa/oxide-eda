---
okf_version: "0.2"
type: Function
title: draw_dof_direction_arrows
description: v0.22 Phase E2 — DOF direction-arrow overlay for under-constrained
resource: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/arrows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/arrows/draw_dof_direction_arrows
language: rust
---

# draw_dof_direction_arrows

v0.22 Phase E2 — DOF direction-arrow overlay for under-constrained

## Signature

```rust
pub(in crate::library::editor::footprint::canvas::draw) fn draw_dof_direction_arrows(
    frame: &mut canvas::Frame,
    cstate: &FootprintCanvasState,
    sketch: &oxide_sketch::SketchData,
    state: &FootprintEditorState,
)
```

## Visibility

- `pub(in crate::library::editor::footprint::canvas::draw)`

## Docstring

v0.22 Phase E2 — DOF direction-arrow overlay for under-constrained
Points. For every Point with `DofColor::Under`, draws a 10-px-long
1-px-wide cyan arrow pointing in the direction of least constraint
sensitivity — i.e. the direction in which moving the Point
increases the constraint residual the least. Visually answers the
"if I drag this blue Point, which way will it go freely?"
question Fusion users expect.

Math: for a Point with Jacobian columns `c_x`, `c_y` (each column
is the partial derivative of every residual w.r.t. that state
var), the direction of greatest constraint sensitivity is the
eigenvector of
`M = [[||c_x||², c_x·c_y], [c_x·c_y, ||c_y||²]]`
associated with the LARGEST eigenvalue. The free-DoF direction is
the perpendicular (smallest-eigenvalue eigenvector).

Closed-form for a 2×2 symmetric matrix:
- λ_min = (a+d)/2 − √(((a-d)/2)² + b²)
- eigenvector for λ_min:
- if |b| > ε: (b, λ_min − a), normalized
- else (already diagonal): pick whichever column is smaller
- if all of a, b, d ≈ 0 (Point isn't touched by any constraint):
default to (1, 0) so the arrow still gives visual feedback.

Hides itself entirely when `state.last_solve` is `None` or the
jacobian is empty.

## Source
Lines 36–155 in `crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/arrows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arrows](/crates/oxide-app/src/library/editor/footprint/canvas/draw/sketch/arrows.md) |
| called_by | [draw_sketch_overlays](/crates/oxide-app/src/library/editor/footprint/canvas/draw/overlays/draw_sketch_overlays.md) |
