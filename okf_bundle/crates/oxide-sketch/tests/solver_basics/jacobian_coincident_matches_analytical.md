---
okf_version: "0.2"
type: Function
title: jacobian_coincident_matches_analytical
description: "[test]"
resource: crates/oxide-sketch/tests/solver_basics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/solver_basics/jacobian_coincident_matches_analytical
language: rust
---

# jacobian_coincident_matches_analytical

[test]

## Signature

```rust
fn jacobian_coincident_matches_analytical()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 162–187 in `crates/oxide-sketch/tests/solver_basics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver_basics](/crates/oxide-sketch/tests/solver_basics.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/solver_basics/empty_params.md) |
| calls | [approx_eq](/crates/oxide-sketch/tests/solver_basics/approx_eq.md) |
