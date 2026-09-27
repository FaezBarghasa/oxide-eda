---
okf_version: "0.2"
type: Function
title: render_other_section
description: v0.18.13 — Other section. Today carries only a Units toggle;
resource: crates/oxide-app/src/panels/footprint_editor_properties/managers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/managers/render_other_section
language: rust
---

# render_other_section

v0.18.13 — Other section. Today carries only a Units toggle;

## Signature

```rust
pub(super) fn render_other_section(
    mut col: Column<'a, PanelMsg>,
    _fp: &'a FootprintEditorPanelContext,
    palette: PanelPalette,
    unit: oxide_types::coord::Unit,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.18.13 — Other section. Today carries only a Units toggle;
future home for additional document-level options.

## Source
Lines 157–206 in `crates/oxide-app/src/panels/footprint_editor_properties/managers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [managers](/crates/oxide-app/src/panels/footprint_editor_properties/managers.md) |
| calls | [form_label](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_label.md) |
| called_by | [view_sections](/crates/oxide-app/src/panels/footprint_editor_properties/sections/view_sections.md) |
