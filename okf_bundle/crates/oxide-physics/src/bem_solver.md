---
okf_version: "0.2"
type: Module
title: bem_solver
description: "2D Boundary Element Method (BEM) Transmission Line & Impedance Field Solver."
resource: crates/oxide-physics/src/bem_solver.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:35:08Z"
concept_id: crates/oxide-physics/src/bem_solver
language: rust
---

# bem_solver

2D Boundary Element Method (BEM) Transmission Line & Impedance Field Solver.

## Docstring

2D Boundary Element Method (BEM) Transmission Line & Impedance Field Solver.

Conforms to Master Technical Directive Horizon II (§3, Task 2.4):
- Electrostatic Boundary Element Method (BEM) solving Laplace's equation in transverse plane.
- Extracts per-unit-length capacitance [C] and inductance [L] matrices.
- Computes characteristic single-ended impedance Z0 and differential impedance Zdiff.

## Relationships

| Type | Target |
|------|--------|
| related | [BemBoundaryElement](/crates/oxide-physics/src/bem_solver/BemBoundaryElement.md) |
| related | [length](/crates/oxide-physics/src/bem_solver/length.md) |
| related | [midpoint](/crates/oxide-physics/src/bem_solver/midpoint.md) |
| related | [length](/crates/oxide-physics/src/bem_solver/length.md) |
| related | [midpoint](/crates/oxide-physics/src/bem_solver/midpoint.md) |
| related | [TransmissionLineCrossSection](/crates/oxide-physics/src/bem_solver/TransmissionLineCrossSection.md) |
| related | [ExtractedTransmissionLine](/crates/oxide-physics/src/bem_solver/ExtractedTransmissionLine.md) |
| related | [BemFieldSolver](/crates/oxide-physics/src/bem_solver/BemFieldSolver.md) |
| related | [solve_microstrip](/crates/oxide-physics/src/bem_solver/solve_microstrip.md) |
| related | [solve_microstrip](/crates/oxide-physics/src/bem_solver/solve_microstrip.md) |
| related | [test_microstrip_50_ohm_extraction](/crates/oxide-physics/src/bem_solver/test_microstrip_50_ohm_extraction.md) |
| related | [test_differential_microstrip_100_ohm_extraction](/crates/oxide-physics/src/bem_solver/test_differential_microstrip_100_ohm_extraction.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
