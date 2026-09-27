---
okf_version: "0.2"
type: Function
title: view_pin_symbol_picker
description: IEEE-symbol pick_list row used four times (Inside / Inside Edge /
resource: crates/oxide-app/src/panels/symbol_editor_properties/pin.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_symbol_picker
language: rust
---

# view_pin_symbol_picker

IEEE-symbol pick_list row used four times (Inside / Inside Edge /

## Signature

```rust
fn view_pin_symbol_picker(
    label: &str,
    current: oxide_library::PinSymbolKind,
    pin_idx: usize,
    slot: u8,
    muted: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Docstring

IEEE-symbol pick_list row used four times (Inside / Inside Edge /
Outside Edge / Outside) on the pin Properties surface. `slot`
matches the `SymEditorSetPinSymbol::slot` numbering: 0 / 1 / 2 / 3.

## Source
Lines 11–71 in `crates/oxide-app/src/panels/symbol_editor_properties/pin.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pin](/crates/oxide-app/src/panels/symbol_editor_properties/pin.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [view_pin_selection](/crates/oxide-app/src/panels/symbol_editor_properties/pin/view_pin_selection.md) |
