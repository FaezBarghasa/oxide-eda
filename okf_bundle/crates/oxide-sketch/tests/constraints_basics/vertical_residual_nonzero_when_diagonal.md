---
okf_version: "0.2"
type: Function
title: vertical_residual_nonzero_when_diagonal
description: "[test]"
resource: crates/oxide-sketch/tests/constraints_basics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/constraints_basics/vertical_residual_nonzero_when_diagonal
language: rust
---

# vertical_residual_nonzero_when_diagonal

[test]

## Signature

```rust
fn vertical_residual_nonzero_when_diagonal()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 217–231 in `crates/oxide-sketch/tests/constraints_basics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints_basics](/crates/oxide-sketch/tests/constraints_basics.md) |
| calls | [pack](/crates/oxide-sketch/src/solver/state/pack.md) |
| calls | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
| calls | [empty_params](/crates/oxide-sketch/tests/constraints_basics/empty_params.md) |
