---
okf_version: "0.2"
type: Function
title: context_menu_action_applies_inner_and_closes_menu
description: "`ContextMenuAction` (routed at the top-level dispatcher, not"
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/context_menu_action_applies_inner_and_closes_menu
language: rust
---

# context_menu_action_applies_inner_and_closes_menu

`ContextMenuAction` (routed at the top-level dispatcher, not

## Signature

```rust
fn context_menu_action_applies_inner_and_closes_menu()
```

## Decorators

- `test`

## Docstring

`ContextMenuAction` (routed at the top-level dispatcher, not
here) applies its boxed action and closes the menu in one step.
[test]

## Source
Lines 307–331 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
