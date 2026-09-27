---
okf_version: "0.2"
type: Function
title: dropdown_overlay
description: Build the dropdown overlay (panel + click-outside backstop) for
resource: crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/active_bar/mod/dropdown_overlay
language: rust
---

# dropdown_overlay

Build the dropdown overlay (panel + click-outside backstop) for

## Signature

```rust
pub fn dropdown_overlay(
    editor: &'a SymbolEditorState,
    theme_id: ThemeId,
    tokens: &'a ThemeTokens,
    top_padding_px: u16,
) -> Option<iced::Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the dropdown overlay (panel + click-outside backstop) for
the currently-open menu. `None` when no menu open.

`top_padding_px`: see [`crate::library::editor::footprint::unified_active_bar::dropdown_overlay`].

## Source
Lines 72–115 in `crates/oxide-app/src/library/editor/symbol/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/symbol/active_bar/mod.md) |
