---
okf_version: "0.2"
type: Function
title: bbox_mm
description: "Un-rotated, axis-aligned half-extent box (min_x, min_y, max_x,"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/bbox_mm_1
language: rust
---

# bbox_mm

Un-rotated, axis-aligned half-extent box (min_x, min_y, max_x,

## Signature

```rust
pub fn bbox_mm(&self) -> (f64, f64, f64, f64)
```

## Visibility

- `pub`

## Docstring

Un-rotated, axis-aligned half-extent box (min_x, min_y, max_x,
max_y) in mm.

This is the PAD-LOCAL frame — `rotation_deg` is deliberately
ignored. Only callers that reason in the pad's own frame want
this (the chamfer / round-rect anchor derivation in
`pad_to_sketch::solve`). Anything asking "where does this pad
actually sit on the board" wants [`Self::rotated_aabb_mm`] or
[`Self::rotated_corners_mm`] instead — reading the un-rotated
box is what left hit-test, courtyard, rubber-band and the pad
renderer all disagreeing with the drawn copper.

## Source
Lines 192–196 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
