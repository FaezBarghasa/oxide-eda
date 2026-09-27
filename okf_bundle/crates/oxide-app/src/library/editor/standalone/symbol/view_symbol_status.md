---
okf_version: "0.2"
type: Function
title: view_symbol_status
description: Bottom status footer for the .snxsym tab — Altium SchLib parity.
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_status
language: rust
---

# view_symbol_status

Bottom status footer for the .snxsym tab — Altium SchLib parity.

## Signature

```rust
fn view_symbol_status(
    editor: &'a SymbolEditorState,
    panel_ctx: &'a PanelContext,
    display: LibraryDisplaySettings,
    path: &'a std::path::Path,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Bottom status footer for the .snxsym tab — Altium SchLib parity.
X / Y in the active unit (mm / mil), zoom %, grid spacing,
pin count + a hint string. Mirrors the global schematic
status bar so a user editing a symbol library has the same
metadata at the same place on screen.

## Source
Lines 68–147 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [format_coord](/crates/oxide-app/src/library/editor/standalone/symbol/format_coord.md) |
| calls | [symbol_tool_button_style](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_tool_button_style.md) |
| calls | [status_bar](/crates/oxide-app/src/styles/status_bar.md) |
| called_by | [view_symbol](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol.md) |
