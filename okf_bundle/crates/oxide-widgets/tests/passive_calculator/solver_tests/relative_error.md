---
okf_version: "0.2"
type: Function
title: relative_error
resource: crates/oxide-widgets/tests/passive_calculator/solver_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/tests/passive_calculator/solver_tests/relative_error
language: rust
---

# relative_error

## Signature

```rust
fn relative_error(value: f64, target: f64) -> f64
```

## Source
Lines 290–292 in `crates/oxide-widgets/tests/passive_calculator/solver_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_tests](/crates/oxide-widgets/tests/passive_calculator/solver_tests.md) |
| called_by | [assert_matches_exhaustive_oracle](/crates/oxide-widgets/tests/passive_calculator/solver_tests/assert_matches_exhaustive_oracle.md) |
| called_by | [exhaustive_best_error](/crates/oxide-widgets/tests/passive_calculator/solver_tests/exhaustive_best_error.md) |
