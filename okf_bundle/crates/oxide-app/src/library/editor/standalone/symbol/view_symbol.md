---
okf_version: "0.2"
type: Function
title: view_symbol
description: "Render the standalone Symbol editor for a `.snxsym` tab. Altium"
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/view_symbol
language: rust
---

# view_symbol

Render the standalone Symbol editor for a `.snxsym` tab. Altium

## Signature

```rust
pub fn view_symbol(
    editor: &'a SymbolEditorState,
    panel_ctx: &'a PanelContext,
    display: LibraryDisplaySettings,
    path: &'a std::path::Path,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Render the standalone Symbol editor for a `.snxsym` tab. Altium
SchLib parity: the canvas takes the full tab width; the right-dock
Properties panel renders symbol/pin properties driven by the
selection (see `panels::view_symbol_editor_properties`). The
in-tab properties column was retired in v0.9 phase 1 so the user
sees the same Properties surface whether editing a schematic or
a symbol library — single source of truth, no duplicated panes.

## Source
Lines 28–61 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [view_symbol_toolbar](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar.md) |
| calls | [view_symbol_canvas](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_canvas.md) |
| calls | [view_symbol_status](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status.md) |
| calls | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| called_by | [view_center](/crates/oxide-app/src/app/view/mod/view_center.md) |
