---
okf_version: "0.2"
type: Function
title: render_sketch_pad_subform
description: v0.21 — Sketch-mode Pad Attributes sub-form. Renders when the
resource: crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad/render_sketch_pad_subform
language: rust
---

# render_sketch_pad_subform

v0.21 — Sketch-mode Pad Attributes sub-form. Renders when the

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_sketch_pad_subform(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
    collapsed_sections: &'a CollapsedSections,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.21 — Sketch-mode Pad Attributes sub-form. Renders when the
selected sketch entity carries a `PadAttr`. Mirrors the
Altium-parity Pad Properties / Pad Stack / Pad Features fields
surfaced for Pads-mode placement, but bound to the sketch
entity's PadAttr rather than the flat-pad list. Geometry-shaping
fields (size_x_expr / size_y_expr / mask_margin_expr / etc.)
stay sketch-parameterised — those are authored via the Sketch
parameter editor, not this form.

## Source
Lines 18–297 in `crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_pad](/crates/oxide-app/src/panels/footprint_editor_properties/subforms/sketch_pad.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [pad_pick_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_pick_row.md) |
| calls | [pad_input_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row.md) |
| calls | [pad_check_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_check_row.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
