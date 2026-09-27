---
okf_version: "0.2"
type: Function
title: bake_paste_apertures
description: Bake every PasteApertureAttr-tagged closed profile.
resource: crates/oxide-bake/src/mask.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/mask/bake_paste_apertures
language: rust
---

# bake_paste_apertures

Bake every PasteApertureAttr-tagged closed profile.

## Signature

```rust
pub fn bake_paste_apertures(
    sketch: &SketchData,
    solve: &FullSolveOutput,
    out: &mut Vec<FpPasteAperture>,
    warnings: &mut Vec<String>,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

Bake every PasteApertureAttr-tagged closed profile.

## Source
Lines 60–75 in `crates/oxide-bake/src/mask.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mask](/crates/oxide-bake/src/mask.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [bake_paste_aperture_rectangle](/crates/oxide-bake/src/mask/bake_paste_aperture_rectangle.md) |
