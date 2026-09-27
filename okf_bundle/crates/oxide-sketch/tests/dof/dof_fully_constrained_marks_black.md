---
okf_version: "0.2"
type: Function
title: dof_fully_constrained_marks_black
description: "[test]"
resource: crates/oxide-sketch/tests/dof.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/dof/dof_fully_constrained_marks_black
language: rust
---

# dof_fully_constrained_marks_black

[test]

## Signature

```rust
fn dof_fully_constrained_marks_black()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 61–112 in `crates/oxide-sketch/tests/dof.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dof](/crates/oxide-sketch/tests/dof.md) |
| calls | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/dof/empty_params.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [numerical_jacobian](/crates/oxide-sketch/src/solver/jacobian/numerical_jacobian.md) |
| calls | [entity_colours](/crates/oxide-sketch/src/solver/dof/entity_colours.md) |
