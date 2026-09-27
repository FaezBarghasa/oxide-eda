---
okf_version: "0.2"
type: Module
title: mask
description: "Mask + paste-aperture bake — turns MaskOpeningAttr,"
resource: crates/oxide-bake/src/mask.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/mask
language: rust
---

# mask

Mask + paste-aperture bake — turns MaskOpeningAttr,

## Docstring

Mask + paste-aperture bake — turns MaskOpeningAttr,
MaskExcludeAttr, and PasteApertureAttr-tagged closed profiles into
`Footprint::mask_openings`, `mask_excludes`, and `paste_apertures`.

Phase B / Stage 3 of the v0.14 sketch-mode plan. All three follow
the same recipe: walker → polygon, attr.layer → LayerId, append to
the corresponding output Vec.

v0.14 scope: Lines only (walker limitation). Arcs / Circles in a
profile surface a warning and skip. Construction entities skipped.

## Relationships

| Type | Target |
|------|--------|
| related | [bake_mask_openings](/crates/oxide-bake/src/mask/bake_mask_openings.md) |
| related | [bake_mask_excludes](/crates/oxide-bake/src/mask/bake_mask_excludes.md) |
| related | [bake_paste_apertures](/crates/oxide-bake/src/mask/bake_paste_apertures.md) |
| related | [bake_layered](/crates/oxide-bake/src/mask/bake_layered.md) |
| related | [solve](/crates/oxide-bake/src/mask/solve.md) |
| related | [rectangle](/crates/oxide-bake/src/mask/rectangle.md) |
| related | [bake_mask_opening_rectangle](/crates/oxide-bake/src/mask/bake_mask_opening_rectangle.md) |
| related | [bake_mask_exclude_rectangle](/crates/oxide-bake/src/mask/bake_mask_exclude_rectangle.md) |
| related | [bake_paste_aperture_rectangle](/crates/oxide-bake/src/mask/bake_paste_aperture_rectangle.md) |
