---
okf_version: "0.2"
type: Function
title: parametric_residual_uses_the_params_the_solve_ran_with
description: "GH #599 — the panel used to re-resolve `sketch.parameters` and"
resource: crates/oxide-app/src/app/runtime/footprint_summaries.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/footprint_summaries/parametric_residual_uses_the_params_the_solve_ran_with
language: rust
---

# parametric_residual_uses_the_params_the_solve_ran_with

GH #599 — the panel used to re-resolve `sketch.parameters` and

## Signature

```rust
fn parametric_residual_uses_the_params_the_solve_ran_with()
```

## Decorators

- `test`

## Docstring

GH #599 — the panel used to re-resolve `sketch.parameters` and
fall back to an empty map when that failed. One unrelated
unparseable parameter was enough: every parameter-driven
constraint then failed to evaluate and was displayed as a
residual of exactly 0.0 — what a satisfied constraint shows.
[test]

## Source
Lines 296–323 in `crates/oxide-app/src/app/runtime/footprint_summaries.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries.md) |
| calls | [two_point_scaffold](/crates/oxide-app/src/app/runtime/footprint_summaries/two_point_scaffold.md) |
| calls | [distance](/crates/oxide-app/src/app/runtime/footprint_summaries/distance.md) |
| calls | [output_for](/crates/oxide-app/src/app/runtime/footprint_summaries/output_for.md) |
| calls | [build_over_constraint_summaries](/crates/oxide-app/src/app/runtime/footprint_summaries/build_over_constraint_summaries.md) |
