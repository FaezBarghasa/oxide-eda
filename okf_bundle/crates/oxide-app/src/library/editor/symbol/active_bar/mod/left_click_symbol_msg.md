---
okf_version: "0.2"
type: Function
title: left_click_symbol_msg
description: "Unwrap a trigger button's left-click message down to the"
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/left_click_symbol_msg
language: rust
---

# left_click_symbol_msg

Unwrap a trigger button's left-click message down to the

## Signature

```rust
fn left_click_symbol_msg(item: &ActiveBarItem<LibraryMessage>) -> &SymbolEditorMsg
```

## Docstring

Unwrap a trigger button's left-click message down to the
`SymbolEditorMsg` it carries — panics on anything but a
`Button` with a `PrimitiveEditorEvent(Symbol(_))` `on_press`.

## Source
Lines 228–243 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
