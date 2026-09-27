---
okf_version: "0.2"
type: Class
title: EditorPad
description: "One pad in the editor canvas. A subset of [`oxide_library::Pad`] —"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/EditorPad
language: rust
---

# EditorPad

One pad in the editor canvas. A subset of [`oxide_library::Pad`] —

## Signature

```rust
pub struct EditorPad
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

One pad in the editor canvas. A subset of [`oxide_library::Pad`] —
we only carry the fields the canvas renders or hit-tests. Extra
fields on `Pad` (drill, mask/paste margins, etc.) round-trip via
[`super::FootprintEditorState::sync_pads_to_primitive`] without a UI yet.
[derive(Debug, Clone, PartialEq)]

## Methods

- `number`
- `position_mm`
- `size_mm`
- `kind`
- `shape`
- `layers`
- `sketch_entity_id`
- `corner_entity_ids`
- `rotation_deg`
- `drill_diameter_mm`
- `stack`
- `feature_top`
- `feature_bottom`
- `testpoint`
- `template`
- `template_library`
- `electrical_type`
- `net`
- `locked`
- `hole_tolerance_plus_mm`
- `hole_tolerance_minus_mm`
- `hole_rotation_deg`
- `copper_offset_x_mm`
- `copper_offset_y_mm`
- `shape_params`

## Source
Lines 17–63 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
