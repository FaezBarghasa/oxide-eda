---
okf_version: "0.2"
type: Function
title: view_symbol_selection
description: "Symbol-level default Properties (nothing selected): identity,"
resource: crates/oxide-app/src/panels/symbol_editor_properties/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/symbol/view_symbol_selection
language: rust
---

# view_symbol_selection

Symbol-level default Properties (nothing selected): identity,

## Signature

```rust
pub(super) fn view_symbol_selection(
    mut col: Column<'a, PanelMsg>,
    sym: &'a SymbolEditorPanelContext,
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

Symbol-level default Properties (nothing selected): identity,
graphical toggles, and local-colour overrides.

## Source
Lines 50–180 in `crates/oxide-app/src/panels/symbol_editor_properties/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/panels/symbol_editor_properties/symbol.md) |
| calls | [prop_row_static](/crates/oxide-app/src/panels/symbol_editor_properties/mod/prop_row_static.md) |
| calls | [local_color_field](/crates/oxide-app/src/panels/symbol_editor_properties/symbol/local_color_field.md) |
| called_by | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
