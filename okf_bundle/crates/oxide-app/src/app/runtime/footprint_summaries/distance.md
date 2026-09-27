---
okf_version: "0.2"
type: Function
title: distance
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/distance
language: rust
---

# distance

## Signature

```rust
fn distance(sketch: &SketchData, target: DimTarget) -> Constraint
```

## Source
Lines 242–251 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| called_by | [parametric_residual_uses_the_params_the_solve_ran_with](/crates/oxide-app/src/app/runtime/footprint_summaries/parametric_residual_uses_the_params_the_solve_ran_with.md) |
| called_by | [unevaluable_residual_is_not_zero_and_sorts_first](/crates/oxide-app/src/app/runtime/footprint_summaries/unevaluable_residual_is_not_zero_and_sorts_first.md) |
