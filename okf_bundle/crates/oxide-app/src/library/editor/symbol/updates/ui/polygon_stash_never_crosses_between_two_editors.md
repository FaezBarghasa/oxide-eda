---
okf_version: "0.2"
type: Function
title: polygon_stash_never_crosses_between_two_editors
description: Regression for the cross-tab corruption class this fix
resource: crates/oxide-app/src/library/editor/symbol/updates/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/ui/polygon_stash_never_crosses_between_two_editors
language: rust
---

# polygon_stash_never_crosses_between_two_editors

Regression for the cross-tab corruption class this fix

## Signature

```rust
fn polygon_stash_never_crosses_between_two_editors()
```

## Decorators

- `test`

## Docstring

Regression for the cross-tab corruption class this fix
replaces (the vertex stash used to live on the canvas
`Program::State`, which iced reuses across `.snxsym` tabs).
The stash now lives on each document's own `SymbolEditorState`,
so two independent editors — standing in for two open tabs —
never share it: placing vertices and switching tools on editor
A must not create, touch, or leak into editor B in any way.
[test]

## Source
Lines 121–170 in `crates/oxide-app/src/library/editor/symbol/updates/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/library/editor/symbol/updates/ui.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/ui/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
