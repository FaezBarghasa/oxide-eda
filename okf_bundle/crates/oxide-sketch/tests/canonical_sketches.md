---
okf_version: "0.2"
type: Module
title: canonical_sketches
description: Task 3.4 — Canonical sketch corpus.
resource: crates/oxide-sketch/tests/canonical_sketches.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/canonical_sketches
language: rust
---

# canonical_sketches

Task 3.4 — Canonical sketch corpus.

## Docstring

Task 3.4 — Canonical sketch corpus.

Four hand-known sketches that exercise the Phase 3.3 LM solver
end-to-end. The fifth sketch from the Task 3.4 spec ("anchored
line, length-constrained") already lives in
`tests/lm_basic.rs::lm_solves_anchored_horizontal_distance` so we
cover the remaining four here:

1. Rectangle (10 × 5).
2. Parallelogram (base 10, side 5, interior angle 60°).
3. Isosceles triangle with 60° apex (i.e. equilateral, side 10).
4. Regular hexagon (circumradius 10).

Coordinate expectations are derived from elementary geometry — no
third-party constraint-solver source / format docs / blog posts /
wiki pages were consulted.

Sign-convention note (`solver/residuals/parallel_perp_angle.rs`):
the `Angle` residual measures `atan2(cross(d1,d2), dot(d1,d2))`,
which is the signed CCW angle FROM `d1` TO `d2` wrapped into
`(−π, π]`. The chosen target signs below are derived by working
out the expected `d1`, `d2` direction vectors at the solution and
plugging into that formula.

## Relationships

| Type | Target |
|------|--------|
| related | [rectangle_10_by_5](/crates/oxide-sketch/tests/canonical_sketches/rectangle_10_by_5.md) |
| related | [parallelogram_base10_side5_60deg](/crates/oxide-sketch/tests/canonical_sketches/parallelogram_base10_side5_60deg.md) |
| related | [isosceles_triangle_apex_60](/crates/oxide-sketch/tests/canonical_sketches/isosceles_triangle_apex_60.md) |
| related | [regular_hexagon_circumradius_10](/crates/oxide-sketch/tests/canonical_sketches/regular_hexagon_circumradius_10.md) |
