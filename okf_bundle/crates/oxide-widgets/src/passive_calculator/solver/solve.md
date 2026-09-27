---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/solve
language: rust
---

# solve

## Signature

```rust
pub fn solve(options: SolveOptions) -> Vec<Network>
```

## Visibility

- `pub`

## Source
Lines 96–145 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [generate_leaves](/crates/oxide-widgets/src/passive_calculator/solver/generate_leaves.md) |
| calls | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| calls | [generate_target_brackets](/crates/oxide-widgets/src/passive_calculator/solver/generate_target_brackets.md) |
| calls | [retain_frontier](/crates/oxide-widgets/src/passive_calculator/solver/retain_frontier.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| calls | [sort_networks](/crates/oxide-widgets/src/passive_calculator/solver/sort_networks.md) |
| calls | [truncate](/crates/oxide-output/examples/qa_harness/truncate.md) |
| called_by | [calculate](/crates/oxide-widgets/src/passive_calculator/control/calculate.md) |
| called_by | [adding_parts_can_improve_the_approximation](/crates/oxide-widgets/tests/passive_calculator/solver_tests/adding_parts_can_improve_the_approximation.md) |
| called_by | [assert_matches_exhaustive_oracle](/crates/oxide-widgets/tests/passive_calculator/solver_tests/assert_matches_exhaustive_oracle.md) |
| called_by | [boundary_targets_return_exact_structured_networks](/crates/oxide-widgets/tests/passive_calculator/solver_tests/boundary_targets_return_exact_structured_networks.md) |
| called_by | [capacitor_solver_uses_capacitor_connection_semantics](/crates/oxide-widgets/tests/passive_calculator/solver_tests/capacitor_solver_uses_capacitor_connection_semantics.md) |
| called_by | [every_preferred_series_can_seed_a_solution](/crates/oxide-widgets/tests/passive_calculator/solver_tests/every_preferred_series_can_seed_a_solution.md) |
| called_by | [exact_single_component_match_is_ranked_first](/crates/oxide-widgets/tests/passive_calculator/solver_tests/exact_single_component_match_is_ranked_first.md) |
| called_by | [extreme_finite_targets_always_return_a_result](/crates/oxide-widgets/tests/passive_calculator/solver_tests/extreme_finite_targets_always_return_a_result.md) |
| called_by | [largest_preferred_series_finds_a_four_part_closest_result](/crates/oxide-widgets/tests/passive_calculator/solver_tests/largest_preferred_series_finds_a_four_part_closest_result.md) |
| called_by | [result_never_exceeds_requested_component_count](/crates/oxide-widgets/tests/passive_calculator/solver_tests/result_never_exceeds_requested_component_count.md) |
| called_by | [solver_is_deterministic](/crates/oxide-widgets/tests/passive_calculator/solver_tests/solver_is_deterministic.md) |
| called_by | [unreachable_target_still_returns_a_closest_network](/crates/oxide-widgets/tests/passive_calculator/solver_tests/unreachable_target_still_returns_a_closest_network.md) |
