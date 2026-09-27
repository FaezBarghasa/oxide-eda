---
okf_version: "0.2"
type: Function
title: materialize_level_three
resource: crates/oxide-widgets/src/passive_calculator/solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-widgets/src/passive_calculator/solver/materialize_level_three
language: rust
---

# materialize_level_three

## Signature

```rust
fn materialize_level_three(
    leaves: &[Network],
    level_two: &[LevelTwoCandidate],
    candidate: LevelThreeCandidate,
) -> Network
```

## Source
Lines 363–373 in `crates/oxide-widgets/src/passive_calculator/solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-widgets/src/passive_calculator/solver.md) |
| calls | [materialize_level_two](/crates/oxide-widgets/src/passive_calculator/solver/materialize_level_two.md) |
| called_by | [find_closest_network](/crates/oxide-widgets/src/passive_calculator/solver/find_closest_network.md) |
