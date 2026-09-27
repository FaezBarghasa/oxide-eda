---
okf_version: "0.2"
type: Function
title: mock_board
resource: crates/oxide-router/tests/router_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:07:42Z"
concept_id: crates/oxide-router/tests/router_tests/mock_board
language: rust
---

# mock_board

## Signature

```rust
fn mock_board() -> PcbBoard
```

## Source
Lines 19–53 in `crates/oxide-router/tests/router_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router_tests](/crates/oxide-router/tests/router_tests.md) |
| called_by | [test_cascade_push_and_shove](/crates/oxide-router/tests/router_tests/test_cascade_push_and_shove.md) |
| called_by | [test_differential_pair_and_meander](/crates/oxide-router/tests/router_tests/test_differential_pair_and_meander.md) |
| called_by | [test_end_to_end_routing_workflow](/crates/oxide-router/tests/router_tests/test_end_to_end_routing_workflow.md) |
| called_by | [test_interactive_astar_routing](/crates/oxide-router/tests/router_tests/test_interactive_astar_routing.md) |
| called_by | [test_ml_guided_astar_routing](/crates/oxide-router/tests/router_tests/test_ml_guided_astar_routing.md) |
| called_by | [test_spatial_index_queries](/crates/oxide-router/tests/router_tests/test_spatial_index_queries.md) |
| called_by | [test_topological_triangulation_and_autoroute](/crates/oxide-router/tests/router_tests/test_topological_triangulation_and_autoroute.md) |
