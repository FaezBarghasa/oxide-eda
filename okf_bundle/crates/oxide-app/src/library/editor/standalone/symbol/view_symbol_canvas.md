---
okf_version: "0.2"
type: Function
title: view_symbol_canvas
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_canvas
language: rust
---

# view_symbol_canvas

## Signature

```rust
fn view_symbol_canvas(
    editor: &'a SymbolEditorState,
    panel_ctx: &'a PanelContext,
    display: LibraryDisplaySettings,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 239–275 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [symbol_action_to_primitive_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_action_to_primitive_msg.md) |
| called_by | [view_symbol](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol.md) |
