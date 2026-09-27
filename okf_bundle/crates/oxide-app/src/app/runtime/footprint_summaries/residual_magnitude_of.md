---
okf_version: "0.2"
type: Function
title: residual_magnitude_of
description: "Residual magnitude for one over-constrained constraint, or `None`"
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/residual_magnitude_of
language: rust
---

# residual_magnitude_of

Residual magnitude for one over-constrained constraint, or `None`

## Signature

```rust
fn residual_magnitude_of(
    c: &oxide_sketch::constraint::Constraint,
    out: &oxide_sketch::solver::FullSolveOutput,
    sketch: &oxide_sketch::sketch::SketchData,
    kind_label: &'static str,
) -> Option<f64>
```

## Docstring

Residual magnitude for one over-constrained constraint, or `None`
when it cannot be evaluated.

GH #599 — this used to substitute `0.0`, which is exactly what a
perfectly satisfied constraint reports, and the descending sort in
[`build_over_constraint_summaries`] then buried the conflict at the
bottom of the diagnostics list. It now returns `None` (which sorts
first) and reports the failure: an unevaluable residual means the
diagnostics the user is reading are incomplete, and a blank cell in
a panel is not a report.

## Source
Lines 138–163 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| called_by | [build_over_constraint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries/build_over_constraint_summaries.md) |
