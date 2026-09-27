---
okf_version: "0.2"
type: Class
title: PadAttr
description: "Attribute attached to a Real Point on a `BoardTopPlane` to"
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/PadAttr
language: rust
---

# PadAttr

Attribute attached to a Real Point on a `BoardTopPlane` to

## Signature

```rust
pub struct PadAttr
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Attribute attached to a Real Point on a `BoardTopPlane` to
indicate that this point bakes to a footprint pad.

`Default` exists so existing literal constructors can omit the
pad-stack / feature / testpoint fields via `..PadAttr::default()`.
Default values place a 1×1 mm SMD round pad on the top side with
no overrides — the canonical "blank" pad.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `number`
- `kind`
- `side`
- `shape`
- `size_x_expr`
- `size_y_expr`
- `rotation_expr`
- `offset_x_expr`
- `offset_y_expr`
- `drill`
- `mask_margin_expr`
- `paste_margin_expr`
- `paste_apertures`
- `template`
- `library`
- `stack`
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
- `owned`

## Source
Lines 228–326 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
