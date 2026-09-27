---
okf_version: "0.2"
type: Class
title: PadFormValues
description: v0.20 — snapshot of the values the form renders. Built from
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/PadFormValues
language: rust
---

# PadFormValues

v0.20 — snapshot of the values the form renders. Built from

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) struct PadFormValues
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.20 — snapshot of the values the form renders. Built from
`FootprintEditorPanelContext.next_pad_*` for placement, or from
`FootprintPadSummary` for the selected-pad branch.
[derive(Debug, Clone)]

## Methods

- `designator`
- `side`
- `rotation_deg`
- `template`
- `template_library`
- `shape`
- `kind`
- `size_x_mm`
- `size_y_mm`
- `drill_diameter_mm`
- `drill_slot_length_mm`
- `stack`
- `feature_top`
- `feature_bottom`
- `testpoint`
- `pad_stack_tab`
- `electrical_type`
- `net`
- `locked`
- `hole_tolerance_plus_mm`
- `hole_tolerance_minus_mm`
- `hole_rotation_deg`
- `copper_offset_x_mm`
- `copper_offset_y_mm`
- `numeric_buffers`

## Source
Lines 34–71 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod.md) |
