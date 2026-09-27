---
okf_version: "0.2"
type: Function
title: view
description: Convenience wrapper — build items + render via
resource: crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pads_active_bar/view
language: rust
---

# view

Convenience wrapper — build items + render via

## Signature

```rust
pub fn view(
    editor: &'a FootprintEditorState,
    theme_id: oxide_types::theme::ThemeId,
    tokens: &'a ThemeTokens,
) -> iced::Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Convenience wrapper — build items + render via
[`oxide_widgets::active_bar::view`].

## Source
Lines 357–363 in `crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pads_active_bar](/crates/oxide-app/src/library/editor/footprint/pads_active_bar.md) |
| calls | [items](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/items.md) |
