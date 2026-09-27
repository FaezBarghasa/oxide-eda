---
okf_version: "0.2"
type: Function
title: push_arc_interior_if_arc
description: "If `entity` is an [`EntityKind::Arc`], append [`ARC_SAMPLES`]"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/push_arc_interior_if_arc
language: rust
---

# push_arc_interior_if_arc

If `entity` is an [`EntityKind::Arc`], append [`ARC_SAMPLES`]

## Signature

```rust
fn push_arc_interior_if_arc(
    out: &mut Vec<[f64; 2]>,
    sketch: &SketchData,
    solve: &FullSolveOutput,
    entity: &Entity,
    from: SketchEntityId,
    to: SketchEntityId,
) -> Result<(), TraceError>
```

## Docstring

If `entity` is an [`EntityKind::Arc`], append [`ARC_SAMPLES`]
interior vertices interpolated along the arc from `from` to `to`.
Lines are no-ops.

Sampling: compute the arc's center, radius, start_angle, end_angle.
Walk N intermediate angles in the direction the arc sweeps from
`from` to `to` (which depends on `sweep_ccw` AND on whether `from`
is the arc's `start` or `end` Point — if the trace approaches the
arc from the `end` side we walk the arc backwards).

## Source
Lines 264–336 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [trace_closed_profile](/crates/oxide-bake/src/profile/trace_closed_profile.md) |
