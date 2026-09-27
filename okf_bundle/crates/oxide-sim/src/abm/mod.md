---
okf_version: "0.2"
type: Module
title: abm
description: "Analog Behavioral Modeling (ABM) & Symbolic Automatic Differentiation Engine."
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod
language: rust
---

# abm

Analog Behavioral Modeling (ABM) & Symbolic Automatic Differentiation Engine.

## Docstring

Analog Behavioral Modeling (ABM) & Symbolic Automatic Differentiation Engine.

Conforms to Master Technical Directive Horizon I (§2, Task 1.4):
- Intermediate Representation (IR) evaluation of non-linear component expressions.
- Symbolic automatic differentiation for exact analytical Jacobians without finite-difference errors.
- Rational Laplace transfer function H(s) transformation into state-space companion forms.

## Relationships

| Type | Target |
|------|--------|
| related | [LaplaceTransferFunction](/crates/oxide-sim/src/abm/mod/LaplaceTransferFunction.md) |
| related | [new](/crates/oxide-sim/src/abm/mod/new.md) |
| related | [evaluate_freq_response](/crates/oxide-sim/src/abm/mod/evaluate_freq_response.md) |
| related | [to_state_space](/crates/oxide-sim/src/abm/mod/to_state_space.md) |
| related | [new](/crates/oxide-sim/src/abm/mod/new.md) |
| related | [evaluate_freq_response](/crates/oxide-sim/src/abm/mod/evaluate_freq_response.md) |
| related | [to_state_space](/crates/oxide-sim/src/abm/mod/to_state_space.md) |
| related | [AbmExpr](/crates/oxide-sim/src/abm/mod/AbmExpr.md) |
| related | [eval](/crates/oxide-sim/src/abm/mod/eval.md) |
| related | [derivative](/crates/oxide-sim/src/abm/mod/derivative.md) |
| related | [eval](/crates/oxide-sim/src/abm/mod/eval.md) |
| related | [derivative](/crates/oxide-sim/src/abm/mod/derivative.md) |
| related | [test_symbolic_differentiation](/crates/oxide-sim/src/abm/mod/test_symbolic_differentiation.md) |
| related | [test_laplace_transfer_function_state_space](/crates/oxide-sim/src/abm/mod/test_laplace_transfer_function_state_space.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
