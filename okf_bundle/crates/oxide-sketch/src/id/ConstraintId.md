---
okf_version: "0.2"
type: Class
title: ConstraintId
description: "[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/id.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/id/ConstraintId
language: rust
---

# ConstraintId

[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Signature

```rust
pub struct ConstraintId
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 28–28 in `crates/oxide-sketch/src/id.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [id](/crates/oxide-sketch/src/id.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [constraint_id_round_trip](/crates/oxide-sketch/tests/round_trip/constraint_id_round_trip.md) |
