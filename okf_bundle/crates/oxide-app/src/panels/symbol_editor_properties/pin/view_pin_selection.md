---
okf_version: "0.2"
type: Function
title: view_pin_selection
description: Pin Properties rows — Designator / Name / Electrical / Position /
resource: crates/oxide-app/src/panels/symbol_editor_properties/pin.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_selection
language: rust
---

# view_pin_selection

Pin Properties rows — Designator / Name / Electrical / Position /

## Signature

```rust
pub(super) fn view_pin_selection(
    mut col: Column<'a, PanelMsg>,
    pin: &'a SymbolPinSummary,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> Column<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Pin Properties rows — Designator / Name / Electrical / Position /
Orientation / Length / metadata toggles / IEEE symbols.

## Source
Lines 75–456 in `crates/oxide-app/src/panels/symbol_editor_properties/pin.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin](/crates/oxide-app/src/panels/symbol_editor_properties/pin.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [view_pin_symbol_picker](/crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_symbol_picker.md) |
| called_by | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
