---
okf_version: "0.2"
type: Function
title: multiply
description: "Composes dual quaternions: $\\hat{\\mathbf{q}}_{\\text{combined}} = \\hat{\\mathbf{q}}_1 \\cdot \\hat{\\mathbf{q}}_2$."
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat/multiply_3
language: rust
---

# multiply

Composes dual quaternions: $\hat{\mathbf{q}}_{\text{combined}} = \hat{\mathbf{q}}_1 \cdot \hat{\mathbf{q}}_2$.

## Signature

```rust
pub fn multiply(&self, other: &Self) -> Self
```

## Visibility

- `pub`

## Docstring

Composes dual quaternions: $\hat{\mathbf{q}}_{\text{combined}} = \hat{\mathbf{q}}_1 \cdot \hat{\mathbf{q}}_2$.

## Source
Lines 82–96 in `crates/oxide-physics/src/dual_quat.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dual_quat](/crates/oxide-physics/src/dual_quat.md) |
