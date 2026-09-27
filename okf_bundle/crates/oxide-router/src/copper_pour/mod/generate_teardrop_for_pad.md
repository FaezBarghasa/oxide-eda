---
okf_version: "0.2"
type: Function
title: generate_teardrop_for_pad
description: Generate a curvilinear/straight teardrop fillet between a track and a circular pad/via.
resource: crates/oxide-router/src/copper_pour/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:53:15Z"
concept_id: crates/oxide-router/src/copper_pour/mod/generate_teardrop_for_pad
language: rust
---

# generate_teardrop_for_pad

Generate a curvilinear/straight teardrop fillet between a track and a circular pad/via.

## Signature

```rust
impl TeardropGenerator { pub fn generate_teardrop_for_pad(
        track_start: Point2D,
        pad_center: Point2D,
        track_width: Microns,
        pad_radius: Microns,
        net_id: NetId,
        layer: LayerId,
    ) -> Option<Vec<RouteSegment>> }
```

## Visibility

- `pub`

## Docstring

Generate a curvilinear/straight teardrop fillet between a track and a circular pad/via.

## Source
Lines 133–185 in `crates/oxide-router/src/copper_pour/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [copper_pour](/crates/oxide-router/src/copper_pour/mod.md) |
