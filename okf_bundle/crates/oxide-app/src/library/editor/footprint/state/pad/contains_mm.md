---
okf_version: "0.2"
type: Function
title: contains_mm
description: "Point-in-pad containment, rotation-aware. Inverse-rotates the"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/contains_mm
language: rust
---

# contains_mm

Point-in-pad containment, rotation-aware. Inverse-rotates the

## Signature

```rust
impl EditorPad { pub fn contains_mm(&self, x: f64, y: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

Point-in-pad containment, rotation-aware. Inverse-rotates the
probe into the pad's own frame and compares against the half
extents, so a turned pad is hit on its real copper rather than
on the axis-aligned box it would occupy unrotated.

## Source
Lines 304–309 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
