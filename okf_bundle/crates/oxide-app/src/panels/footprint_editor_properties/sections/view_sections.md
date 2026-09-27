---
okf_version: "0.2"
type: Function
title: view_sections
resource: crates/oxide-app/src/panels/footprint_editor_properties/sections.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections
language: rust
---

# view_sections

## Signature

```rust
pub(super) fn view_sections(
    mut col: Column<'a, PanelMsg>,
    fp: &'a FootprintEditorPanelContext,
    palette: PanelPalette,
    collapsed_sections: &'a CollapsedSections,
    unit: oxide_types::coord::Unit,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Source
Lines 12–531 in `crates/oxide-app/src/panels/footprint_editor_properties/sections.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| calls | [render_snapping_mode_row](/crates/oxide-app/src/panels/footprint_editor_properties/snap_options/render_snapping_mode_row.md) |
| calls | [render_grid_manager](/crates/oxide-app/src/panels/footprint_editor_properties/managers/render_grid_manager.md) |
| calls | [render_other_section](/crates/oxide-app/src/panels/footprint_editor_properties/managers/render_other_section.md) |
| calls | [props_kv_row](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_kv_row.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
