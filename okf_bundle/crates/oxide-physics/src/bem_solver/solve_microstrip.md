---
okf_version: "0.2"
type: Function
title: solve_microstrip
description: Evaluates transmission line parameters using closed-form analytical conformal mapping / BEM formulations.
resource: crates/oxide-physics/src/bem_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:35:08Z"
concept_id: crates/oxide-physics/src/bem_solver/solve_microstrip
language: rust
---

# solve_microstrip

Evaluates transmission line parameters using closed-form analytical conformal mapping / BEM formulations.

## Signature

```rust
impl BemFieldSolver { pub fn solve_microstrip(cross_section: &TransmissionLineCrossSection) -> ExtractedTransmissionLine }
```

## Visibility

- `pub`

## Docstring

Evaluates transmission line parameters using closed-form analytical conformal mapping / BEM formulations.

## Source
Lines 64–112 in `crates/oxide-physics/src/bem_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bem_solver](/crates/oxide-physics/src/bem_solver.md) |
