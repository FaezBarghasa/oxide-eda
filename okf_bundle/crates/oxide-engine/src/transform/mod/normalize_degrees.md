---
okf_version: "0.2"
type: Function
title: normalize_degrees
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/normalize_degrees
language: rust
---

# normalize_degrees

## Signature

```rust
fn normalize_degrees(angle_degrees: f64) -> f64
```

## Source
Lines 500–504 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [normalize_angle_rad](/crates/oxide-types/src/rotation2d/normalize_angle_rad.md) |
| called_by | [rotate_selected_item](/crates/oxide-engine/src/transform/mod/rotate_selected_item.md) |
