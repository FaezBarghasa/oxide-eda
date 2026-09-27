---
okf_version: "0.2"
type: Module
title: pdn
description: "Power Delivery Network (PDN) Impedance Field Solving & Decoupling Optimization."
resource: crates/oxide-rf/src/pdn.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-rf"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:41:24Z"
concept_id: crates/oxide-rf/src/pdn
language: rust
---

# pdn

Power Delivery Network (PDN) Impedance Field Solving & Decoupling Optimization.

## Docstring

Power Delivery Network (PDN) Impedance Field Solving & Decoupling Optimization.

Conforms to Master Technical Directive §6.1:
- Target Impedance Synthesis: $Z_{\text{target}} = \frac{V_{\text{dd}} \cdot \Delta V_{\text{ripple}}}{I_{\text{transient}}}$
- Planar cavity resonance model for power/ground plane impedance $Z_{\text{PDN}}(f)$ across DC to 10 GHz.
- Decoupling optimization evaluating ESL, ESR, capacitance, and mounting via inductance.

## Relationships

| Type | Target |
|------|--------|
| related | [PdnTargetSpec](/crates/oxide-rf/src/pdn/PdnTargetSpec.md) |
| related | [new](/crates/oxide-rf/src/pdn/new.md) |
| related | [compute_target_impedance](/crates/oxide-rf/src/pdn/compute_target_impedance.md) |
| related | [new](/crates/oxide-rf/src/pdn/new.md) |
| related | [compute_target_impedance](/crates/oxide-rf/src/pdn/compute_target_impedance.md) |
| related | [DecapModel](/crates/oxide-rf/src/pdn/DecapModel.md) |
| related | [new](/crates/oxide-rf/src/pdn/new.md) |
| related | [impedance_at](/crates/oxide-rf/src/pdn/impedance_at.md) |
| related | [new](/crates/oxide-rf/src/pdn/new.md) |
| related | [impedance_at](/crates/oxide-rf/src/pdn/impedance_at.md) |
| related | [PowerPlaneCavity](/crates/oxide-rf/src/pdn/PowerPlaneCavity.md) |
| related | [static_capacitance](/crates/oxide-rf/src/pdn/static_capacitance.md) |
| related | [static_capacitance](/crates/oxide-rf/src/pdn/static_capacitance.md) |
| related | [PdnSolver](/crates/oxide-rf/src/pdn/PdnSolver.md) |
| related | [evaluate_impedance_profile](/crates/oxide-rf/src/pdn/evaluate_impedance_profile.md) |
| related | [verify_compliance](/crates/oxide-rf/src/pdn/verify_compliance.md) |
| related | [evaluate_impedance_profile](/crates/oxide-rf/src/pdn/evaluate_impedance_profile.md) |
| related | [verify_compliance](/crates/oxide-rf/src/pdn/verify_compliance.md) |
| related | [test_pdn_target_impedance_calculation](/crates/oxide-rf/src/pdn/test_pdn_target_impedance_calculation.md) |
| related | [test_pdn_impedance_profile_evaluation](/crates/oxide-rf/src/pdn/test_pdn_impedance_profile_evaluation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
