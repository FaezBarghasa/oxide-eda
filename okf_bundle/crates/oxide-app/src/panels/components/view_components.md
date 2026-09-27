---
okf_version: "0.2"
type: Function
title: view_components
description: ─── Components Panel (matched to Altium Designer) ───────────
resource: crates/oxide-app/src/panels/components.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/components/view_components
language: rust
---

# view_components

─── Components Panel (matched to Altium Designer) ───────────

## Signature

```rust
pub fn view_components(ctx: &'a PanelContext) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

─── Components Panel (matched to Altium Designer) ───────────

## Source
Lines 8–294 in `crates/oxide-app/src/panels/components.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components](/crates/oxide-app/src/panels/components.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [selection_color](/crates/oxide-widgets/src/theme_ext/selection_color.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [section_hdr](/crates/oxide-app/src/panels/properties_parameters/form_rows/section_hdr.md) |
| calls | [form_input_row](/crates/oxide-app/src/panels/properties_parameters/form_rows/form_input_row.md) |
| calls | [symbol_preview](/crates/oxide-widgets/src/symbol_preview/symbol_preview.md) |
| called_by | [view_panel](/crates/oxide-app/src/panels/mod/view_panel.md) |
