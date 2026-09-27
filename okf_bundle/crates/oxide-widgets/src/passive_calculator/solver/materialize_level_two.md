---
okf_version: "0.2"
type: Function
title: materialize_level_two
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/materialize_level_two
language: rust
---

# materialize_level_two

## Signature

```rust
fn materialize_level_two(leaves: &[Network], candidate: LevelTwoCandidate) -> Network
```

## Source
Lines 355–361 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
| called_by | [materialize_level_three](/crates/oxide-widgets/src/passive_calculator/solver/materialize_level_three.md) |
