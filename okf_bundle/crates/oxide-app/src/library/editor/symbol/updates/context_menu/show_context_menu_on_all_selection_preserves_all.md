---
okf_version: "0.2"
type: Function
title: show_context_menu_on_all_selection_preserves_all
description: "Right-click on any graphic while `All` is selected preserves"
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_all_selection_preserves_all
language: rust
---

# show_context_menu_on_all_selection_preserves_all

Right-click on any graphic while `All` is selected preserves

## Signature

```rust
fn show_context_menu_on_all_selection_preserves_all()
```

## Decorators

- `test`

## Docstring

Right-click on any graphic while `All` is selected preserves
`All` — it already covers every pin and graphic.
[test]

## Source
Lines 204–218 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
