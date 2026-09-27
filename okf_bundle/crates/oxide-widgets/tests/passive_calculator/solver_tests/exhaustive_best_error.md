---
okf_version: "0.2"
type: Function
title: exhaustive_best_error
resource: crates/oxide-widgets/tests/passive_calculator/solver_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/tests/passive_calculator/solver_tests/exhaustive_best_error
language: rust
---

# exhaustive_best_error

## Signature

```rust
fn exhaustive_best_error(
    series: ESeries,
    kind: ComponentKind,
    target: f64,
    max_parts: usize,
) -> f64
```

## Source
Lines 225–265 in `crates/oxide-widgets/tests/passive_calculator/solver_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_tests](/crates/oxide-widgets/tests/passive_calculator/solver_tests.md) |
| calls | [additive_value](/crates/oxide-widgets/tests/passive_calculator/solver_tests/additive_value.md) |
| calls | [harmonic_value](/crates/oxide-widgets/tests/passive_calculator/solver_tests/harmonic_value.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| calls | [relative_error](/crates/oxide-widgets/tests/passive_calculator/solver_tests/relative_error.md) |
| called_by | [assert_matches_exhaustive_oracle](/crates/oxide-widgets/tests/passive_calculator/solver_tests/assert_matches_exhaustive_oracle.md) |
