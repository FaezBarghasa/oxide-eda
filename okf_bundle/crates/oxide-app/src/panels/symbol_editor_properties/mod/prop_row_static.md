---
okf_version: "0.2"
type: Function
title: prop_row_static
description: Read-only label + value Properties row shared by the symbol-level
resource: crates/oxide-app/src/panels/symbol_editor_properties/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/mod/prop_row_static
language: rust
---

# prop_row_static

Read-only label + value Properties row shared by the symbol-level

## Signature

```rust
fn prop_row_static(
    label: &str,
    value: String,
    muted: Color,
    primary: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

Read-only label + value Properties row shared by the symbol-level
and field selections.

## Source
Lines 111–133 in `crates/oxide-app/src/panels/symbol_editor_properties/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod.md) |
| called_by | [view_symbol_editor_properties](/crates/oxide-app/src/panels/symbol_editor_properties/mod/view_symbol_editor_properties.md) |
| called_by | [view_symbol_selection](/crates/oxide-app/src/panels/symbol_editor_properties/symbol/view_symbol_selection.md) |
