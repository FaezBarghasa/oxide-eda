---
okf_version: "0.2"
type: Function
title: view_footprint_editor_properties
description: v0.14.2 — Properties panel body for the Footprint editor. Switches
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties
language: rust
---

# view_footprint_editor_properties

v0.14.2 — Properties panel body for the Footprint editor. Switches

## Signature

```rust
pub(super) fn view_footprint_editor_properties(
    fp: &'a FootprintEditorPanelContext,
    palette: PanelPalette,
    custom_filter_presets: Vec<crate::active_bar::CustomFilterPreset>,
    active_custom_filter_tab: usize,
    collapsed_sections: &'a CollapsedSections,
    unit: oxide_types::coord::Unit,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.14.2 — Properties panel body for the Footprint editor. Switches
between three contexts:

1. **Pads mode + pad selected** — pad number, kind, shape, size,
position, layer count.
2. **Sketch mode + entity selected** — entity kind, position
(Points only), construction flag, attached-constraint count.
3. **Default** (any mode, no selection) — footprint summary
(name + version), counts (pads, sketch entities, constraints),
and the most recent solve summary when a sketch exists.

## Source
Lines 38–214 in `crates/oxide-app/src/panels/footprint_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod.md) |
| calls | [render_pad_form_properties](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_properties.md) |
| calls | [props_kv_row](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row.md) |
| calls | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
| calls | [render_pad_form_pad_features](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/mod/render_pad_form_pad_features.md) |
| calls | [view_selection](/crates/oxide-app/src/panels/footprint_editor_properties/selection/view_selection.md) |
| calls | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
| calls | [render_fp_settings_and_hint](/crates/oxide-app/src/panels/footprint_editor_properties/mod/render_fp_settings_and_hint.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
