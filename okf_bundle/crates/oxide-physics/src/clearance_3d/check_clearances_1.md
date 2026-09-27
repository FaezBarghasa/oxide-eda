---
okf_version: "0.2"
type: Function
title: check_clearances
description: Check 3D component-to-component and component-to-enclosure clearances.
resource: crates/oxide-physics/src/clearance_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:09:20Z"
concept_id: crates/oxide-physics/src/clearance_3d/check_clearances_1
language: rust
---

# check_clearances

Check 3D component-to-component and component-to-enclosure clearances.

## Signature

```rust
pub fn check_clearances(
        bodies: &[Body3d],
        min_clearance_mm: f64,
        enclosure_max_height_mm: Option<f64>,
    ) -> Vec<ClearanceViolation3d>
```

## Visibility

- `pub`

## Docstring

Check 3D component-to-component and component-to-enclosure clearances.

## Source
Lines 114–166 in `crates/oxide-physics/src/clearance_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clearance_3d](/crates/oxide-physics/src/clearance_3d.md) |
