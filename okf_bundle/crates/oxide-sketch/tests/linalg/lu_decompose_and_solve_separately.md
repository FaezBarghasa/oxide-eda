---
okf_version: "0.2"
type: Function
title: lu_decompose_and_solve_separately
description: "[test]"
resource: crates/oxide-sketch/tests/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/linalg/lu_decompose_and_solve_separately
language: rust
---

# lu_decompose_and_solve_separately

[test]

## Signature

```rust
fn lu_decompose_and_solve_separately()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 256–277 in `crates/oxide-sketch/tests/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/tests/linalg.md) |
| calls | [lu_decompose](/crates/oxide-sketch/src/solver/linalg/lu_decompose.md) |
| calls | [lu_solve](/crates/oxide-sketch/src/solver/linalg/lu_solve.md) |
| calls | [assert_vec_close](/crates/oxide-sketch/tests/linalg/assert_vec_close.md) |
| calls | [mat_vec](/crates/oxide-sketch/tests/linalg/mat_vec.md) |
