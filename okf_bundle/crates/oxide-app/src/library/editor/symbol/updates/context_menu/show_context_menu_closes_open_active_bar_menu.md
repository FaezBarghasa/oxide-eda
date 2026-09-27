---
okf_version: "0.2"
type: Function
title: show_context_menu_closes_open_active_bar_menu
description: "`ShowContextMenu` closes any open active-bar dropdown first —"
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_closes_open_active_bar_menu
language: rust
---

# show_context_menu_closes_open_active_bar_menu

`ShowContextMenu` closes any open active-bar dropdown first —

## Signature

```rust
fn show_context_menu_closes_open_active_bar_menu()
```

## Decorators

- `test`

## Docstring

`ShowContextMenu` closes any open active-bar dropdown first —
two popups never coexist (footprint parity).
[test]

## Source
Lines 242–257 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
