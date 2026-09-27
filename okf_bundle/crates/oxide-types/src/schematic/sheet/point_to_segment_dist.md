---
okf_version: "0.2"
type: Function
title: point_to_segment_dist
description: Distance from a point to a line segment.
resource: crates/oxide-types/src/schematic/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-types/src/schematic/sheet/point_to_segment_dist
language: rust
---

# point_to_segment_dist

Distance from a point to a line segment.

## Signature

```rust
pub fn point_to_segment_dist(px: f64, py: f64, ax: f64, ay: f64, bx: f64, by: f64) -> f64
```

## Visibility

- `pub`

## Docstring

Distance from a point to a line segment.

## Source
Lines 403–415 in `crates/oxide-types/src/schematic/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-types/src/schematic/sheet.md) |
