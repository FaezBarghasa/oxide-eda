---
okf_version: "0.2"
type: Function
title: view_symbol_editor_properties
description: "Properties panel content for the active `.snxsym` standalone editor"
resource: crates/oxide-app/src/panels/symbol_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties
language: rust
---

# view_symbol_editor_properties

Properties panel content for the active `.snxsym` standalone editor

## Signature

```rust
pub(super) fn view_symbol_editor_properties(
    sym: &'a SymbolEditorPanelContext,
    muted: Color,
    primary: Color,
    border_c: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Properties panel content for the active `.snxsym` standalone editor
tab. Mirrors Altium SchLib's right-dock Properties.

## Source
Lines 21–107 in `crates/oxide-app/src/panels/symbol_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod.md) |
| calls | [view_symbol_selection](/crates/oxide-app/src/panels/symbol_editor_properties/symbol/view_symbol_selection.md) |
| calls | [view_pin_selection](/crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_selection.md) |
| calls | [view_graphic_selection](/crates/oxide-app/src/panels/symbol_editor_properties/graphic/view_graphic_selection.md) |
| calls | [prop_row_static](/crates/oxide-app/src/panels/symbol_editor_properties/mod/prop_row_static.md) |
| called_by | [view_properties](/crates/oxide-app/src/panels/properties/view_properties.md) |
