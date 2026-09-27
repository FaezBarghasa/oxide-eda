---
okf_version: "0.2"
type: Module
title: dof
description: Task 3.5 — DOF colour classification tests.
resource: crates/oxide-sketch/tests/dof.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/dof
language: rust
---

# dof

Task 3.5 — DOF colour classification tests.

## Docstring

Task 3.5 — DOF colour classification tests.

Three canonical cases:
- **Under**-constrained: a free Point with no constraints lands
`DofColor::Under`.
- **Full**-constrained: the Phase 3.3 anchored line case (Fixed
P1 + Distance + Horizontal) lands both endpoints on `Full`.
- **Over**-constrained: two conflicting Distance constraints on
the same point pair are flagged in `over_constraint_ids` and
bump the touched Point to `DofColor::Over`.

## Relationships

| Type | Target |
|------|--------|
| related | [empty_params](/crates/oxide-sketch/tests/dof/empty_params.md) |
| related | [dof_under_constrained_marks_blue](/crates/oxide-sketch/tests/dof/dof_under_constrained_marks_blue.md) |
| related | [dof_fully_constrained_marks_black](/crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black.md) |
| related | [dof_over_constrained_marks_red](/crates/oxide-sketch/tests/dof/dof_over_constrained_marks_red.md) |
| related | [dof_parametric_over_constraint_marks_red](/crates/oxide-sketch/tests/dof/dof_parametric_over_constraint_marks_red.md) |
