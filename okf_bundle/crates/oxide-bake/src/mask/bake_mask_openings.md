---
okf_version: "0.2"
type: Function
title: bake_mask_openings
description: Bake every MaskOpeningAttr-tagged closed profile.
resource: crates/oxide-bake/src/mask.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/mask/bake_mask_openings
language: rust
---

# bake_mask_openings

Bake every MaskOpeningAttr-tagged closed profile.

## Signature

```rust
pub fn bake_mask_openings(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<FpMaskOpening>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake every MaskOpeningAttr-tagged closed profile.

## Source
Lines 24–39 in `crates/oxide-bake/src/mask.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mask](/crates/oxide-bake/src/mask.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_mask_opening_rectangle](/crates/oxide-bake/src/mask/bake_mask_opening_rectangle.md) |
