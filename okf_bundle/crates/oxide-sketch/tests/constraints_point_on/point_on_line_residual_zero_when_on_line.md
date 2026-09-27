---
okf_version: "0.2"
type: Function
title: point_on_line_residual_zero_when_on_line
description: "[test]"
resource: crates/oxide-sketch/tests/constraints_point_on.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/constraints_point_on/point_on_line_residual_zero_when_on_line
language: rust
---

# point_on_line_residual_zero_when_on_line

[test]

## Signature

```rust
fn point_on_line_residual_zero_when_on_line()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 20–36 in `crates/oxide-sketch/tests/constraints_point_on.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints_point_on](/crates/oxide-sketch/tests/constraints_point_on.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/constraints_point_on/empty_params.md) |
