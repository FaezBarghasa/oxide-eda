---
okf_version: "0.2"
type: Function
title: render_pad_form_pad_stack
description: "v0.20 — render the \"Pad Stack\" section: copper shape + size,"
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack
language: rust
---

# render_pad_form_pad_stack

v0.20 — render the "Pad Stack" section: copper shape + size,

## Signature

```rust
pub(in crate::panels::footprint_editor_properties) fn render_pad_form_pad_stack(
    mut col: Column<'a, PanelMsg>,
    values: &PadFormValues,
    target: PadEditTarget,
    palette: PanelPalette,
    collapsed_sections: &'a CollapsedSections,
    shape_params: &'a [crate::panels::PadShapeParamSummary],
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(in crate::panels::footprint_editor_properties)`

## Docstring

v0.20 — render the "Pad Stack" section: copper shape + size,
hole, per-side paste / mask expansions, tented flags, thermal
relief, corner radius. Mirrors the Altium PCB Library Pad Stack
section: a stylized 2D preview at the top, a Simple/Top-Middle-
Bottom/Full Stack tab strip, then the field rows. The tabs are
UI-only structure today; per-layer overrides require a v0.21
schema follow-up so all three tabs render the same content.

v0.24 Phase 3 (Track A2) — `shape_params` carries the linked
sketch-parameter handles (e.g. `"corner_r"` / `"diameter"`) for
the selected pad. Each entry renders an editable text-input row
reading / writing the live sketch parameter expression so the
user can drive parametric pad geometry from the Properties panel
without entering Sketch mode. Empty for pads with no parametric
handles (Rect / Oval) and during pad placement (no minted
entities yet).

## Source
Lines 34–487 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [pad_stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_preview.md) |
| calls | [pad_stack_tab_strip](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/pad_stack_tab_strip.md) |
| calls | [pad_table_header](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_header.md) |
| calls | [pad_copper_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_copper_row.md) |
| calls | [pad_input_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/pad_input_row.md) |
| calls | [pad_table_row](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_row.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
| called_by | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
