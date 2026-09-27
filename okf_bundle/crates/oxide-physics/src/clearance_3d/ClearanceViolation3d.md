---
okf_version: "0.2"
type: Class
title: ClearanceViolation3d
description: 3D Clearance and Collision violation report.
resource: crates/oxide-physics/src/clearance_3d.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:09:20Z"
concept_id: crates/oxide-physics/src/clearance_3d/ClearanceViolation3d
language: rust
---

# ClearanceViolation3d

3D Clearance and Collision violation report.

## Signature

```rust
pub struct ClearanceViolation3d
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

3D Clearance and Collision violation report.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `item_a`
- `item_b`
- `actual_clearance_mm`
- `required_clearance_mm`
- `is_collision`

## Source
Lines 101–107 in `crates/oxide-physics/src/clearance_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clearance_3d](/crates/oxide-physics/src/clearance_3d.md) |
