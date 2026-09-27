---
okf_version: "0.2"
type: Function
title: distance_to
description: Compute shortest 3D distance between two non-overlapping AABBs.
resource: crates/oxide-physics/src/clearance_3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:09:20Z"
concept_id: crates/oxide-physics/src/clearance_3d/distance_to_3
language: rust
---

# distance_to

Compute shortest 3D distance between two non-overlapping AABBs.

## Signature

```rust
pub fn distance_to(&self, other: &Self) -> f64
```

## Visibility

- `pub`

## Docstring

Compute shortest 3D distance between two non-overlapping AABBs.
Returns 0.0 if they overlap.

## Source
Lines 53–79 in `crates/oxide-physics/src/clearance_3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clearance_3d](/crates/oxide-physics/src/clearance_3d.md) |
