---
okf_version: "0.2"
type: Function
title: symbol_action_to_primitive_msg
resource: crates/oxide-app/src/library/editor/standalone/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/symbol/symbol_action_to_primitive_msg
language: rust
---

# symbol_action_to_primitive_msg

## Signature

```rust
fn symbol_action_to_primitive_msg(action: sym_canvas::CanvasAction) -> SymbolEditorMsg
```

## Source
Lines 277–356 in `crates/oxide-app/src/library/editor/standalone/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/library/editor/standalone/symbol.md) |
| calls | [symbol_selection_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_selection_to_msg.md) |
| calls | [graphic_handle_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/graphic_handle_to_msg.md) |
| calls | [rotate_pivot_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/rotate_pivot_to_msg.md) |
| calls | [symbol_context_target_to_msg](/crates/oxide-app/src/library/editor/standalone/symbol/symbol_context_target_to_msg.md) |
| called_by | [view_symbol_canvas](/crates/oxide-app/src/library/editor/standalone/symbol/view_symbol_canvas.md) |
