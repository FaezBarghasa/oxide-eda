---
okf_version: "0.2"
type: Function
title: two_point_scaffold
description: "Two Points 5 mm apart, ready for the caller to attach"
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/two_point_scaffold
language: rust
---

# two_point_scaffold

Two Points 5 mm apart, ready for the caller to attach

## Signature

```rust
fn two_point_scaffold() -> SketchData
```

## Docstring

Two Points 5 mm apart, ready for the caller to attach
`DistancePtPt` constraints between them.

## Source
Lines 222–240 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| called_by | [parametric_residual_uses_the_params_the_solve_ran_with](/crates/oxide-app/src/app/runtime/footprint_summaries/parametric_residual_uses_the_params_the_solve_ran_with.md) |
| called_by | [unevaluable_residual_is_not_zero_and_sorts_first](/crates/oxide-app/src/app/runtime/footprint_summaries/unevaluable_residual_is_not_zero_and_sorts_first.md) |
