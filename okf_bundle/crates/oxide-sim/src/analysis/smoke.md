---
okf_version: "0.2"
type: Module
title: smoke
description: "Component Stress & Smoke Analysis (Safe Operating Area - SOA)."
resource: crates/oxide-sim/src/analysis/smoke.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:29:34Z"
concept_id: crates/oxide-sim/src/analysis/smoke
language: rust
---

# smoke

Component Stress & Smoke Analysis (Safe Operating Area - SOA).

## Docstring

Component Stress & Smoke Analysis (Safe Operating Area - SOA).

Conforms to Master Technical Directive §3.7:
Real-time monitoring of instantaneous and time-averaged stresses against manufacturer limit databases:
$\text{Stress Ratio} = \max(V_{\text{peak}}/V_{\text{breakdown}}, I_{\text{RMS}}/I_{\text{max}}, P_{\text{avg}}/P_{\text{rated}}, T_j/T_{j, \text{max}})$

## Relationships

| Type | Target |
|------|--------|
| related | [ComponentLimits](/crates/oxide-sim/src/analysis/smoke/ComponentLimits.md) |
| related | [default](/crates/oxide-sim/src/analysis/smoke/default.md) |
| related | [default](/crates/oxide-sim/src/analysis/smoke/default.md) |
| related | [SimulatedStress](/crates/oxide-sim/src/analysis/smoke/SimulatedStress.md) |
| related | [StressEvaluation](/crates/oxide-sim/src/analysis/smoke/StressEvaluation.md) |
| related | [SmokeAnalyzer](/crates/oxide-sim/src/analysis/smoke/SmokeAnalyzer.md) |
| related | [evaluate_stress](/crates/oxide-sim/src/analysis/smoke/evaluate_stress.md) |
| related | [evaluate_stress](/crates/oxide-sim/src/analysis/smoke/evaluate_stress.md) |
| related | [test_smoke_analysis_within_safe_area](/crates/oxide-sim/src/analysis/smoke/test_smoke_analysis_within_safe_area.md) |
| related | [test_smoke_analysis_voltage_violation](/crates/oxide-sim/src/analysis/smoke/test_smoke_analysis_voltage_violation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
