---
okf_version: "0.2"
type: Function
title: build_dimension_input
description: "Build the inline dimension `text_input` slot. Sized to read like"
resource: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/build_dimension_input
language: rust
---

# build_dimension_input

Build the inline dimension `text_input` slot. Sized to read like

## Signature

```rust
fn build_dimension_input(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Element<'static, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

Build the inline dimension `text_input` slot. Sized to read like
the rest of the bar (matches the BTN_SIZE vertical rhythm).

## Source
Lines 530–581 in `crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [items](/crates/oxide-app/src/library/editor/footprint/sketch_mode/active_bar/items.md) |
