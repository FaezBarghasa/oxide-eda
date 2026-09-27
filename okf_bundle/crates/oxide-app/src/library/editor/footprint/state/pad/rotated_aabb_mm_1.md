---
okf_version: "0.2"
type: Function
title: rotated_aabb_mm
description: "Axis-aligned bounding box of the ROTATED pad, in mm. Equals"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/rotated_aabb_mm_1
language: rust
---

# rotated_aabb_mm

Axis-aligned bounding box of the ROTATED pad, in mm. Equals

## Signature

```rust
pub fn rotated_aabb_mm(&self) -> (f64, f64, f64, f64)
```

## Visibility

- `pub`

## Docstring

Axis-aligned bounding box of the ROTATED pad, in mm. Equals
[`Self::bbox_mm`] at zero rotation and grows to enclose the
turned copper otherwise.

## Source
Lines 253–262 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
