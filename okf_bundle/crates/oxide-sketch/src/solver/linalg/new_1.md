---
okf_version: "0.2"
type: Function
title: new
description: Factor a square matrix into its packed LU form. The input is
resource: crates/oxide-sketch/src/solver/linalg.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/linalg/new_1
language: rust
---

# new

Factor a square matrix into its packed LU form. The input is

## Signature

```rust
pub fn new(a: &[Vec<f64>]) -> Result<Self, LinAlgError>
```

## Visibility

- `pub`

## Docstring

Factor a square matrix into its packed LU form. The input is
borrowed and cloned internally so callers can reuse `a`.

## Source
Lines 145–149 in `crates/oxide-sketch/src/solver/linalg.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [linalg](/crates/oxide-sketch/src/solver/linalg.md) |
| calls | [lu_decompose](/crates/oxide-sketch/src/solver/linalg/lu_decompose.md) |
