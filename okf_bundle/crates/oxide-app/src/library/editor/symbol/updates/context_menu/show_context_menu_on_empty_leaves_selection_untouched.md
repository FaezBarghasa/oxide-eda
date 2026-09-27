---
okf_version: "0.2"
type: Function
title: show_context_menu_on_empty_leaves_selection_untouched
description: Right-click on bare canvas opens the menu at the given coords
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_empty_leaves_selection_untouched
language: rust
---

# show_context_menu_on_empty_leaves_selection_untouched

Right-click on bare canvas opens the menu at the given coords

## Signature

```rust
fn show_context_menu_on_empty_leaves_selection_untouched()
```

## Decorators

- `test`

## Docstring

Right-click on bare canvas opens the menu at the given coords
with `Empty` target and doesn't touch the current selection.
[test]

## Source
Lines 112–132 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| calls | [Pin](/crates/oxide-types/src/schematic/mod/Pin.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
