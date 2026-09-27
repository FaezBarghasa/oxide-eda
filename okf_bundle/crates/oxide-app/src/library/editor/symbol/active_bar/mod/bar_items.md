---
okf_version: "0.2"
type: Function
title: bar_items
description: Build the SchLib bar items only — caller mounts via
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/bar_items
language: rust
---

# bar_items

Build the SchLib bar items only — caller mounts via

## Signature

```rust
pub fn bar_items(
    editor: &SymbolEditorState,
    theme_id: ThemeId,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Visibility

- `pub`

## Docstring

Build the SchLib bar items only — caller mounts via
`oxide_widgets::active_bar::view(items, tokens)` so the chain is
identical to the schematic.

## Source
Lines 33–66 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
| calls | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_trigger_items.md) |
