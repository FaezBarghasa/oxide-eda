---
okf_version: "0.2"
type: Function
title: show_context_menu_on_multiple_member_preserves_selection
description: "Right-click on a graphic that's a member of the current"
resource: crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/context_menu/show_context_menu_on_multiple_member_preserves_selection
language: rust
---

# show_context_menu_on_multiple_member_preserves_selection

Right-click on a graphic that's a member of the current

## Signature

```rust
fn show_context_menu_on_multiple_member_preserves_selection()
```

## Decorators

- `test`

## Docstring

Right-click on a graphic that's a member of the current
`Multiple` selection preserves the whole selection — the
headline flow (box-select several lines, right-click one,
Join into Polygon) must not collapse down to a single graphic.
[test]

## Source
Lines 159–177 in `crates/oxide-app/src/library/editor/symbol/updates/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/new_editor.md) |
| calls | [apply_symbol_context_menu](/crates/oxide-app/src/library/editor/symbol/updates/context_menu/apply_symbol_context_menu.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
