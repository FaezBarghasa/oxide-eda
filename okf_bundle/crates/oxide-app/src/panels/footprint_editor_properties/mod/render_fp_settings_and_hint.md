---
okf_version: "0.2"
type: Function
title: render_fp_settings_and_hint
description: v0.20 — common Settings + Hint footer. Always renders the
resource: crates/oxide-app/src/panels/footprint_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/mod/render_fp_settings_and_hint
language: rust
---

# render_fp_settings_and_hint

v0.20 — common Settings + Hint footer. Always renders the

## Signature

```rust
fn render_fp_settings_and_hint(
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

## Docstring

v0.20 — common Settings + Hint footer. Always renders the
Auto-fit Courtyard toggle and a mode-specific hint string.

## Source
Lines 218–263 in `crates/oxide-app/src/panels/footprint_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod.md) |
| calls | [props_section_header](/crates/oxide-app/src/panels/footprint_editor_properties/mod/props_section_header.md) |
| calls | [fp_is_collapsed](/crates/oxide-app/src/panels/footprint_editor_properties/mod/fp_is_collapsed.md) |
| called_by | [view_footprint_editor_properties](/crates/oxide-app/src/panels/footprint_editor_properties/mod/view_footprint_editor_properties.md) |
