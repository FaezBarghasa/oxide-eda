---
okf_version: "0.2"
type: Module
title: solver
description: "End-to-end solver acceptance for `split_line` (issue #360 blocker 3):"
resource: crates/oxide-sketch/src/split/tests/solver.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/tests/solver
language: rust
---

# solver

End-to-end solver acceptance for `split_line` (issue #360 blocker 3):

## Docstring

End-to-end solver acceptance for `split_line` (issue #360 blocker 3):
a split followed by a re-solve must converge, and the duplicated
`Horizontal` / `Vertical` carry-over must pull a perturbed mid
Point back onto the line without disturbing untouched geometry.

## Relationships

| Type | Target |
|------|--------|
| related | [perturbed_rectangle](/crates/oxide-sketch/src/split/tests/solver/perturbed_rectangle.md) |
| related | [split_then_solve_leaves_rectangle_visually_unchanged](/crates/oxide-sketch/src/split/tests/solver/split_then_solve_leaves_rectangle_visually_unchanged.md) |
