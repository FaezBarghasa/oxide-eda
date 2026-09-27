---
okf_version: "0.2"
type: Function
title: options
resource: crates/oxide-widgets/tests/passive_calculator/solver_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/tests/passive_calculator/solver_tests/options
language: rust
---

# options

## Signature

```rust
fn options(kind: ComponentKind, target: f64, max_parts: usize) -> SolveOptions
```

## Source
Lines 6–15 in `crates/oxide-widgets/tests/passive_calculator/solver_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_tests](/crates/oxide-widgets/tests/passive_calculator/solver_tests.md) |
| called_by | [adding_parts_can_improve_the_approximation](/crates/oxide-widgets/tests/passive_calculator/solver_tests/adding_parts_can_improve_the_approximation.md) |
| called_by | [boundary_targets_return_exact_structured_networks](/crates/oxide-widgets/tests/passive_calculator/solver_tests/boundary_targets_return_exact_structured_networks.md) |
| called_by | [capacitor_solver_uses_capacitor_connection_semantics](/crates/oxide-widgets/tests/passive_calculator/solver_tests/capacitor_solver_uses_capacitor_connection_semantics.md) |
| called_by | [every_preferred_series_can_seed_a_solution](/crates/oxide-widgets/tests/passive_calculator/solver_tests/every_preferred_series_can_seed_a_solution.md) |
| called_by | [exact_single_component_match_is_ranked_first](/crates/oxide-widgets/tests/passive_calculator/solver_tests/exact_single_component_match_is_ranked_first.md) |
| called_by | [largest_preferred_series_finds_a_four_part_closest_result](/crates/oxide-widgets/tests/passive_calculator/solver_tests/largest_preferred_series_finds_a_four_part_closest_result.md) |
| called_by | [result_never_exceeds_requested_component_count](/crates/oxide-widgets/tests/passive_calculator/solver_tests/result_never_exceeds_requested_component_count.md) |
| called_by | [solver_is_deterministic](/crates/oxide-widgets/tests/passive_calculator/solver_tests/solver_is_deterministic.md) |
| called_by | [unreachable_target_still_returns_a_closest_network](/crates/oxide-widgets/tests/passive_calculator/solver_tests/unreachable_target_still_returns_a_closest_network.md) |
