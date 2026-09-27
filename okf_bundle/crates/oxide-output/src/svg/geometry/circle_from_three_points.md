---
okf_version: "0.2"
type: Function
title: circle_from_three_points
description: Circle through three SVG-space points. Delegates to the canonical
resource: crates/oxide-output/src/svg/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-output/src/svg/geometry/circle_from_three_points
language: rust
---

# circle_from_three_points

Circle through three SVG-space points. Delegates to the canonical

## Signature

```rust
fn circle_from_three_points(a: SvgPoint, b: SvgPoint, c: SvgPoint) -> Option<(f32, f32, f32)>
```

## Docstring

Circle through three SVG-space points. Delegates to the canonical
`oxide_types::schematic::circumcircle` (#483) so the SVG arc exporter
shares the one collinearity epsilon instead of carrying a 5th copy of
the formula. SVG geometry is `f32`; the math runs in `f64` (as it
already did inline here) and casts back — byte-identical to the former
hand-rolled version.

## Source
Lines 112–120 in `crates/oxide-output/src/svg/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-output/src/svg/geometry.md) |
| calls | [circumcircle](/crates/oxide-types/src/schematic/mod/circumcircle.md) |
| called_by | [arc_path_commands](/crates/oxide-output/src/svg/geometry/arc_path_commands.md) |
