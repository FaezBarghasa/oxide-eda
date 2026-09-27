---
okf_version: "0.2"
type: Function
title: lm_no_constraints_returns_immediately
description: "[test]"
resource: crates/oxide-sketch/tests/lm_basic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/lm_basic/lm_no_constraints_returns_immediately
language: rust
---

# lm_no_constraints_returns_immediately

[test]

## Signature

```rust
fn lm_no_constraints_returns_immediately()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 111–124 in `crates/oxide-sketch/tests/lm_basic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lm_basic](/crates/oxide-sketch/tests/lm_basic.md) |
| calls | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/lm_basic/empty_params.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
