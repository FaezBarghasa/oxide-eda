---
okf_version: "0.2"
type: Class
title: SketchPadAttrSummary
description: v0.18.24 — Read-only summary of the currently-selected silk-front
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/SketchPadAttrSummary
language: rust
---

# SketchPadAttrSummary

v0.18.24 — Read-only summary of the currently-selected silk-front

## Signature

```rust
pub struct SketchPadAttrSummary
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

v0.18.24 — Read-only summary of the currently-selected silk-front
v0.21 — sketch-mode pad attribute snapshot. Mirrors the new
fields we added to `PadAttr` so the sketch-entity Properties
branch can render an editable Pad Attributes section.
[derive(Debug, Clone)]

## Methods

- `id`
- `electrical_type`
- `net`
- `locked`
- `template`
- `template_library`
- `feature_top`
- `feature_bottom`
- `testpoint`
- `thermal_relief`
- `mask_top_tented`
- `mask_bottom_tented`
- `paste_top_enabled`
- `paste_bottom_enabled`
- `corner_radius_pct`
- `hole_tolerance_plus_mm`
- `hole_tolerance_minus_mm`
- `hole_rotation_deg`
- `copper_offset_x_mm`
- `copper_offset_y_mm`
- `has_drill`

## Source
Lines 551–575 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
