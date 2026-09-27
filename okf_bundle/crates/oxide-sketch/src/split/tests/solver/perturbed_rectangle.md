---
okf_version: "0.2"
type: Function
title: perturbed_rectangle
description: "Perturbed axis-aligned rectangle — `p1` Fixed at the origin, width"
resource: crates/oxide-sketch/src/split/tests/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/tests/solver/perturbed_rectangle
language: rust
---

# perturbed_rectangle

Perturbed axis-aligned rectangle — `p1` Fixed at the origin, width

## Signature

```rust
fn perturbed_rectangle() -> (
    SketchData,
    SketchEntityId,
    SketchEntityId,
    SketchEntityId,
    SketchEntityId,
    SketchEntityId,
)
```

## Docstring

Perturbed axis-aligned rectangle — `p1` Fixed at the origin, width
10 mm, height 5 mm — plus the constraint set that pins it there:
Horizontal on the bottom/top edges, Vertical on the left/right
edges, and one `DistancePtPt` per axis. 6 residuals over 6 free
scalars (`p1` is Fixed and excluded from the state vector) — the
system is exactly determined, so it converges to a unique solution
near the perturbed initial guess rather than trivially no-op-ing.
Returns `(sketch, p1, p2, p3, p4, l1)`; `l1` (the bottom edge,
`p1 -> p2`) is the one the test below splits.

## Source
Lines 19–86 in `crates/oxide-sketch/src/split/tests/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-sketch/src/split/tests/solver.md) |
| called_by | [split_then_solve_leaves_rectangle_visually_unchanged](/crates/oxide-sketch/src/split/tests/solver/split_then_solve_leaves_rectangle_visually_unchanged.md) |
