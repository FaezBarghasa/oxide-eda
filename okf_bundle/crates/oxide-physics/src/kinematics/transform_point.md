---
okf_version: "0.2"
type: Function
title: transform_point
resource: crates/oxide-physics/src/kinematics.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:09:22Z"
concept_id: crates/oxide-physics/src/kinematics/transform_point
language: rust
---

# transform_point

## Signature

```rust
impl Matrix4x4 { pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] }
```

## Visibility

- `pub`

## Source
Lines 87–97 in `crates/oxide-physics/src/kinematics.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kinematics](/crates/oxide-physics/src/kinematics.md) |
