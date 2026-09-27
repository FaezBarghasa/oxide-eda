---
okf_version: "0.2"
type: Function
title: entity_point_xy
description: "Coordinates of a `Point` entity, or `None` if `id` doesn't resolve"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/entity_point_xy
language: rust
---

# entity_point_xy

Coordinates of a `Point` entity, or `None` if `id` doesn't resolve

## Signature

```rust
fn entity_point_xy(sketch: &SketchData, id: SketchEntityId) -> Option<(f64, f64)>
```

## Docstring

Coordinates of a `Point` entity, or `None` if `id` doesn't resolve
or resolves to a non-`Point` entity.

## Source
Lines 331–340 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| called_by | [point_param](/crates/oxide-sketch/src/split/constraints/point_param.md) |
| called_by | [validate_split](/crates/oxide-sketch/src/split/mod/validate_split.md) |
| called_by | [mid_split_creates_two_lines_and_drops_original](/crates/oxide-sketch/src/split/tests/carry_over/mid_split_creates_two_lines_and_drops_original.md) |
| called_by | [split_at_non_half_t_interpolates_correctly](/crates/oxide-sketch/src/split/tests/carry_over/split_at_non_half_t_interpolates_correctly.md) |
| called_by | [a_realistic_close_to_end_split_still_succeeds](/crates/oxide-sketch/src/split/tests/errors/a_realistic_close_to_end_split_still_succeeds.md) |
