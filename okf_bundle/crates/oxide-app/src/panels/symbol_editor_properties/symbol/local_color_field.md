---
okf_version: "0.2"
type: Function
title: local_color_field
description: Render one Local Colors row (Fills / Lines / Pins) via the shared
resource: crates/oxide-app/src/panels/symbol_editor_properties/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/symbol/local_color_field
language: rust
---

# local_color_field

Render one Local Colors row (Fills / Lines / Pins) via the shared

## Signature

```rust
fn local_color_field(
    label: &'a str,
    slot: crate::app::LocalColorSlot,
    current: Option<[u8; 4]>,
    muted: Color,
    border_c: Color,
    picker: Option<crate::app::LocalColorPicker>,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Render one Local Colors row (Fills / Lines / Pins) via the shared
[`color_field`] widget. `picker` carries the transient open-state;
the inline palette / HSV overlay only expands when it targets this
slot. `None` = inherit from the sheet palette.

## Source
Lines 12–46 in `crates/oxide-app/src/panels/symbol_editor_properties/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/panels/symbol_editor_properties/symbol.md) |
| calls | [color_field](/crates/oxide-app/src/panels/color_field/color_field.md) |
| called_by | [view_symbol_selection](/crates/oxide-app/src/panels/symbol_editor_properties/symbol/view_symbol_selection.md) |
