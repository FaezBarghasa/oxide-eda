---
okf_version: "0.2"
type: Function
title: a_failed_solve_clears_the_previous_solves_colours
description: "#613 — a failed solve must not leave the previous solve's answer"
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/a_failed_solve_clears_the_previous_solves_colours
language: rust
---

# a_failed_solve_clears_the_previous_solves_colours

#613 — a failed solve must not leave the previous solve's answer

## Signature

```rust
fn a_failed_solve_clears_the_previous_solves_colours()
```

## Decorators

- `test`

## Docstring

#613 — a failed solve must not leave the previous solve's answer
on the canvas. Everything that paints constraint colours,
dimensions and DOF state reads `state.last_solve`, so a stale
`Some` is the canvas confidently showing a solution that no
longer describes the sketch. This is the case #610's
`entity_colours` fix cannot reach: the conflict touches a free
point, the solver returns `DidNotConverge`, and `entity_colours`
never runs at all.
[test]

## Source
Lines 134–195 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| calls | [empty_footprint](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/empty_footprint.md) |
| calls | [point_with_pad](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_with_pad.md) |
| calls | [apply_sketch_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit.md) |
