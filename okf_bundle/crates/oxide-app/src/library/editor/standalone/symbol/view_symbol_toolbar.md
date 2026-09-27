---
okf_version: "0.2"
type: Function
title: view_symbol_toolbar
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_toolbar
language: rust
---

# view_symbol_toolbar

## Signature

```rust
fn view_symbol_toolbar(
    editor: &'a SymbolEditorState,
    panel_ctx: &'a PanelContext,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 166–218 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [symbol_tool_button_style](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_tool_button_style.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| calls | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
| called_by | [view_symbol](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol.md) |
