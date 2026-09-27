---
okf_version: "0.2"
type: Function
title: matmul_ata
description: "`Aᵀ · A` for an `m × n` matrix `A`. Returns the `n × n` Gram"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/matmul_ata
language: rust
---

# matmul_ata

`Aᵀ · A` for an `m × n` matrix `A`. Returns the `n × n` Gram

## Signature

```rust
pub fn matmul_ata(a: &[Vec<f64>]) -> Vec<Vec<f64>>
```

## Visibility

- `pub`

## Docstring

`Aᵀ · A` for an `m × n` matrix `A`. Returns the `n × n` Gram
matrix as `Vec<Vec<f64>>` (row-major). The result is symmetric;
only the upper triangle is computed and the lower triangle is
mirrored to avoid redundant arithmetic.

LM's normal equations need this on every iteration: `(AᵀA + λI)
Δx = −Aᵀr`.

## Source
Lines 181–202 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [matmul_ata_2x2](/crates/oxide-sketch/src/solver/math/matmul_ata_2x2.md) |
| called_by | [matmul_ata_is_symmetric](/crates/oxide-sketch/src/solver/math/matmul_ata_is_symmetric.md) |
