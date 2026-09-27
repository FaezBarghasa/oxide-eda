---
okf_version: "0.2"
type: Function
title: leaf_entry
description: "A real-action row — wraps `row.msg` in `ContextMenuAction` so the"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/mod/leaf_entry
language: rust
---

# leaf_entry

A real-action row — wraps `row.msg` in `ContextMenuAction` so the

## Signature

```rust
fn leaf_entry(row: SymbolMenuRow, path: &Path, indented: bool) -> DropdownEntry<LibraryMessage>
```

## Docstring

A real-action row — wraps `row.msg` in `ContextMenuAction` so the
dispatcher applies it and closes the menu in one step, and
disables the row (no `on_press`) when `!row.enabled`.

## Source
Lines 130–143 in `crates/oxide-app/src/library/editor/symbol/context_menu/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/symbol/context_menu/mod.md) |
| calls | [wrap](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/wrap.md) |
| called_by | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
