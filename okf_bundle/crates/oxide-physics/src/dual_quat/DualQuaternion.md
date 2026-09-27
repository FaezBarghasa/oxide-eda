---
okf_version: "0.2"
type: Class
title: DualQuaternion
description: "Unit Dual Quaternion $\\hat{\\mathbf{q}} = \\mathbf{q}_r + \\epsilon \\mathbf{q}_d$."
resource: crates/oxide-physics/src/dual_quat.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:36:07Z"
concept_id: crates/oxide-physics/src/dual_quat/DualQuaternion
language: rust
---

# DualQuaternion

Unit Dual Quaternion $\hat{\mathbf{q}} = \mathbf{q}_r + \epsilon \mathbf{q}_d$.

## Signature

```rust
pub struct DualQuaternion
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Unit Dual Quaternion $\hat{\mathbf{q}} = \mathbf{q}_r + \epsilon \mathbf{q}_d$.
[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]

## Methods

- `real`
- `dual`

## Source
Lines 60–63 in `crates/oxide-physics/src/dual_quat.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dual_quat](/crates/oxide-physics/src/dual_quat.md) |
