---
okf_version: "0.2"
type: Function
title: items
description: Build the Pads-mode Active Bar items.
resource: crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pads_active_bar/items
language: rust
---

# items

Build the Pads-mode Active Bar items.

## Signature

```rust
pub fn items(
    editor: &FootprintEditorState,
    theme_id: oxide_types::theme::ThemeId,
) -> Vec<ActiveBarItem<LibraryMessage>>
```

## Visibility

- `pub`

## Docstring

Build the Pads-mode Active Bar items.

## Source
Lines 249–353 in `crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pads_active_bar](/crates/oxide-app/src/library/editor/footprint/pads_active_bar.md) |
| called_by | [view](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/view.md) |
