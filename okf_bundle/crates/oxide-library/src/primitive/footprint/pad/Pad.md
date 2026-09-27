---
okf_version: "0.2"
type: Class
title: Pad
description: One PCB pad.
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/Pad
language: rust
---

# Pad

One PCB pad.

## Signature

```rust
pub struct Pad
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One PCB pad.

`Default` exists so existing literal constructors can omit the
pad-stack / feature / testpoint fields via `..Pad::default()`.
Default values place a 0×0 mm round SMD pad at the origin with
no overrides — the canonical "blank" pad. Real callers always
override the geometry fields explicitly.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `number`
- `kind`
- `shape`
- `size`
- `position`
- `rotation`
- `layers`
- `drill`
- `solder_mask_margin`
- `paste_margin`
- `template`
- `template_library`
- `paste_margin_top`
- `paste_margin_bottom`
- `paste_enabled_top`
- `paste_enabled_bottom`
- `mask_margin_top`
- `mask_margin_bottom`
- `mask_tented_top`
- `mask_tented_bottom`
- `thermal_relief`
- `corner_radius_pct`
- `feature_top`
- `feature_bottom`
- `testpoint`
- `electrical_type`
- `net`
- `locked`
- `hole_tolerance_plus_mm`
- `hole_tolerance_minus_mm`
- `hole_rotation_deg`
- `copper_offset_x_mm`
- `copper_offset_y_mm`

## Source
Lines 135–239 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
