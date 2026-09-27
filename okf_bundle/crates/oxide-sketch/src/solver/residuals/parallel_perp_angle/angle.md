---
okf_version: "0.2"
type: Function
title: angle
description: "Angle: signed CCW angle from `d1` to `d2` equals `target_rad`."
resource: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/angle
language: rust
---

# angle

Angle: signed CCW angle from `d1` to `d2` equals `target_rad`.

## Signature

```rust
pub fn angle(
    l1: SketchEntityId,
    l2: SketchEntityId,
    target_rad: f64,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

Angle: signed CCW angle from `d1` to `d2` equals `target_rad`.

Residual = `wrap_to_pi(atan2(cross, dot) − target_rad)` mapped into
`(−π, π]` so the LM driver sees a continuous derivative across a
sketch that crosses the ±π branch cut.

## Source
Lines 73–85 in `crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parallel_perp_angle](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle.md) |
| calls | [line_dir](/crates/oxide-sketch/src/solver/residuals/parallel_perp_angle/line_dir.md) |
