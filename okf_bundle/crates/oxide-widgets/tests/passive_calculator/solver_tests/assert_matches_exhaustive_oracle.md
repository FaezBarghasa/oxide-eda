---
okf_version: "0.2"
type: Function
title: assert_matches_exhaustive_oracle
resource: crates/oxide-widgets/tests/passive_calculator/solver_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/tests/passive_calculator/solver_tests/assert_matches_exhaustive_oracle
language: rust
---

# assert_matches_exhaustive_oracle

## Signature

```rust
fn assert_matches_exhaustive_oracle(
    series: ESeries,
    kind: ComponentKind,
    target: f64,
    max_parts: usize,
)
```

## Source
Lines 200–223 in `crates/oxide-widgets/tests/passive_calculator/solver_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_tests](/crates/oxide-widgets/tests/passive_calculator/solver_tests.md) |
| calls | [solve](/crates/oxide-widgets/src/passive_calculator/solver/solve.md) |
| calls | [exhaustive_best_error](/crates/oxide-widgets/tests/passive_calculator/solver_tests/exhaustive_best_error.md) |
| calls | [relative_error](/crates/oxide-widgets/tests/passive_calculator/solver_tests/relative_error.md) |
| called_by | [optimized_search_matches_an_exhaustive_small_series_oracle](/crates/oxide-widgets/tests/passive_calculator/solver_tests/optimized_search_matches_an_exhaustive_small_series_oracle.md) |
