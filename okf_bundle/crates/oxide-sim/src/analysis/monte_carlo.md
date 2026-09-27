---
okf_version: "0.2"
type: Module
title: monte_carlo
description: "Deterministic Monte Carlo & Statistical Distribution Engine."
resource: crates/oxide-sim/src/analysis/monte_carlo.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:03Z"
concept_id: crates/oxide-sim/src/analysis/monte_carlo
language: rust
---

# monte_carlo

Deterministic Monte Carlo & Statistical Distribution Engine.

## Docstring

Deterministic Monte Carlo & Statistical Distribution Engine.

Conforms to Master Technical Directive §3.7:
Parameter variations (Gaussian, Uniform, Lognormal, and correlated component lots)
generated via deterministic pseudo-random seeds guaranteeing bit-exact waveform reproducibility.

## Relationships

| Type | Target |
|------|--------|
| related | [ParameterDistribution](/crates/oxide-sim/src/analysis/monte_carlo/ParameterDistribution.md) |
| related | [TolerancedParameter](/crates/oxide-sim/src/analysis/monte_carlo/TolerancedParameter.md) |
| related | [MonteCarloRun](/crates/oxide-sim/src/analysis/monte_carlo/MonteCarloRun.md) |
| related | [DeterministicPrng](/crates/oxide-sim/src/analysis/monte_carlo/DeterministicPrng.md) |
| related | [new](/crates/oxide-sim/src/analysis/monte_carlo/new.md) |
| related | [next_u64](/crates/oxide-sim/src/analysis/monte_carlo/next_u64.md) |
| related | [next_f64](/crates/oxide-sim/src/analysis/monte_carlo/next_f64.md) |
| related | [next_gaussian](/crates/oxide-sim/src/analysis/monte_carlo/next_gaussian.md) |
| related | [new](/crates/oxide-sim/src/analysis/monte_carlo/new.md) |
| related | [next_u64](/crates/oxide-sim/src/analysis/monte_carlo/next_u64.md) |
| related | [next_f64](/crates/oxide-sim/src/analysis/monte_carlo/next_f64.md) |
| related | [next_gaussian](/crates/oxide-sim/src/analysis/monte_carlo/next_gaussian.md) |
| related | [MonteCarloEngine](/crates/oxide-sim/src/analysis/monte_carlo/MonteCarloEngine.md) |
| related | [generate_runs](/crates/oxide-sim/src/analysis/monte_carlo/generate_runs.md) |
| related | [generate_runs](/crates/oxide-sim/src/analysis/monte_carlo/generate_runs.md) |
| related | [test_deterministic_monte_carlo_reproducibility](/crates/oxide-sim/src/analysis/monte_carlo/test_deterministic_monte_carlo_reproducibility.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
