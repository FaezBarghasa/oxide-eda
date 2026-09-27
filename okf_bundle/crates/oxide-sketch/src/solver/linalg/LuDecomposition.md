---
okf_version: "0.2"
type: Class
title: LuDecomposition
description: "Bundled LU factorisation: the packed `LU` matrix and the row-pivot"
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/LuDecomposition
language: rust
---

# LuDecomposition

Bundled LU factorisation: the packed `LU` matrix and the row-pivot

## Signature

```rust
pub struct LuDecomposition
```

## Visibility

- `pub`

## Docstring

Bundled LU factorisation: the packed `LU` matrix and the row-pivot
trail produced by [`lu_decompose`]. Factor-once, solve-many API
shape: factor `(JᵀJ + λI)` once, then call [`LuDecomposition::solve`]
repeatedly against different right-hand sides without recomputing the
factorisation.

LM uses this in the inner loop: factor `(JᵀJ + λI)` once per
iteration and solve against `−Jᵀr` without recomputing the
factorisation if the step is rejected and `λ` updates.

## Methods

- `lu`
- `perm`

## Source
Lines 137–140 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
