---
okf_version: "0.2"
type: Function
title: view
description: Convenience wrapper — build items + render via
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/view
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
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Convenience wrapper — build items + render via
[`oxide_widgets::active_bar::view`].

## Source
Lines 361–367 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.md) |
| calls | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
