---
okf_version: "0.2"
type: Function
title: make_matrix
description: "Build a well-conditioned `n × n` matrix that exercises pivoting:"
resource: crates/oxide-sketch/examples/bench_linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/examples/bench_linalg/make_matrix
language: rust
---

# make_matrix

Build a well-conditioned `n × n` matrix that exercises pivoting:

## Signature

```rust
fn make_matrix(n: usize) -> (Vec<Vec<f64>>, Vec<f64>)
```

## Docstring

Build a well-conditioned `n × n` matrix that exercises pivoting:
strongly diagonally dominant with off-diagonals decaying away
from the diagonal. The system is solvable to machine precision.

## Source
Lines 52–66 in `crates/oxide-sketch/examples/bench_linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bench_linalg](/crates/oxide-sketch/examples/bench_linalg.md) |
| called_by | [main](/crates/oxide-sketch/examples/bench_linalg/main.md) |
