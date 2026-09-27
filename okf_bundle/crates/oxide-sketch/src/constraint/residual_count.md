---
okf_version: "0.2"
type: Function
title: residual_count
description: Number of scalar residuals this constraint contributes.
resource: crates/oxide-sketch/src/constraint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/constraint/residual_count
language: rust
---

# residual_count

Number of scalar residuals this constraint contributes.

## Signature

```rust
impl ConstraintKind { pub fn residual_count(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Number of scalar residuals this constraint contributes.

## Source
Lines 123–146 in `crates/oxide-sketch/src/constraint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraint](/crates/oxide-sketch/src/constraint.md) |
