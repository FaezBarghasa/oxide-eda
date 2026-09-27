---
okf_version: "0.2"
type: Function
title: solve
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/solve
language: rust
---

# solve

## Signature

```rust
fn solve(sketch: &SketchData) -> FullSolveOutput
```

## Source
Lines 438–442 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [trace_arc_seed_walks_back_through_line](/crates/oxide-bake/src/profile/trace_arc_seed_walks_back_through_line.md) |
| called_by | [trace_branching_topology_errors](/crates/oxide-bake/src/profile/trace_branching_topology_errors.md) |
| called_by | [trace_construction_lines_skipped](/crates/oxide-bake/src/profile/trace_construction_lines_skipped.md) |
| called_by | [trace_d_shape_cw_arc_closes_lower_half](/crates/oxide-bake/src/profile/trace_d_shape_cw_arc_closes_lower_half.md) |
| called_by | [trace_d_shape_line_plus_arc_closes](/crates/oxide-bake/src/profile/trace_d_shape_line_plus_arc_closes.md) |
| called_by | [trace_open_chain_returns_open_error](/crates/oxide-bake/src/profile/trace_open_chain_returns_open_error.md) |
| called_by | [trace_rectangle_closes](/crates/oxide-bake/src/profile/trace_rectangle_closes.md) |
