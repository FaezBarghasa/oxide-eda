---
okf_version: "0.2"
type: Module
title: lm_basic
description: Task 3.3 — Levenberg–Marquardt iteration smoke tests.
resource: crates/oxide-sketch/tests/lm_basic.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/lm_basic
language: rust
---

# lm_basic

Task 3.3 — Levenberg–Marquardt iteration smoke tests.

## Docstring

Task 3.3 — Levenberg–Marquardt iteration smoke tests.

Phase 3.4 ships the full canonical-sketch corpus (rectangle,
parallelogram, isosceles triangle, regular hexagon). This file
covers the minimum cases needed to verify LM is correct on the
anchored-line case from the plan.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_params](/crates/oxide-sketch/tests/lm_basic/empty_params.md) |
| related | [lm_solves_anchored_horizontal_distance](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_horizontal_distance.md) |
| related | [lm_solves_anchored_distance_in_either_direction](/crates/oxide-sketch/tests/lm_basic/lm_solves_anchored_distance_in_either_direction.md) |
| related | [lm_no_constraints_returns_immediately](/crates/oxide-sketch/tests/lm_basic/lm_no_constraints_returns_immediately.md) |
| related | [lm_already_converged_returns_quickly](/crates/oxide-sketch/tests/lm_basic/lm_already_converged_returns_quickly.md) |
| related | [lm_solves_offset_circle_via_distance_pt_circle](/crates/oxide-sketch/tests/lm_basic/lm_solves_offset_circle_via_distance_pt_circle.md) |
