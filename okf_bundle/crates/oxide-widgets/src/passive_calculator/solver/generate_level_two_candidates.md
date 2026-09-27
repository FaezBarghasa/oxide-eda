---
okf_version: "0.2"
type: Function
title: generate_level_two_candidates
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/generate_level_two_candidates
language: rust
---

# generate_level_two_candidates

## Signature

```rust
fn generate_level_two_candidates(
    leaves: &[Network],
    kind: ComponentKind,
) -> Vec<LevelTwoCandidate>
```

## Source
Lines 232–256 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [combine_values](/crates/oxide-widgets/src/passive_calculator/solver/combine_values.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
