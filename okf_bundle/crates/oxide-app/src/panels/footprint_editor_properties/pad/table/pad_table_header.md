---
okf_version: "0.2"
type: Function
title: pad_table_header
description: v0.20 — Altium-style table header row. Renders the column titles
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/table/pad_table_header
language: rust
---

# pad_table_header

v0.20 — Altium-style table header row. Renders the column titles

## Signature

```rust
pub(super) fn pad_table_header(
    cols: &[&'static str],
    portions: &[u16],
    muted: Color,
    border_c: Color,
) -> iced::Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

v0.20 — Altium-style table header row. Renders the column titles
in muted small text with the same FillPortion layout the data
rows use, so columns line up vertically. First cell is the
section family name (COPPER / HOLE / PASTE / SOLDER).

## Source
Lines 21–54 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/table.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [table](/crates/oxide-app/src/panels/footprint_editor_properties/pad/table.md) |
| called_by | [render_pad_form_pad_stack](/crates/oxide-app/src/panels/footprint_editor_properties/pad/form/stack/render_pad_form_pad_stack.md) |
