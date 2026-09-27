---
okf_version: "0.2"
type: Function
title: footprint_tabs_overlay
description: v0.18.7 — multi-footprint tab strip. Renders one button per
resource: crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pads_active_bar/footprint_tabs_overlay
language: rust
---

# footprint_tabs_overlay

v0.18.7 — multi-footprint tab strip. Renders one button per

## Signature

```rust
pub fn footprint_tabs_overlay(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> iced::Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

v0.18.7 — multi-footprint tab strip. Renders one button per
footprint inside the active `.snxfpt` envelope plus a trailing
"+" button that appends a new sibling. The active sibling paints
with the accent background. Hidden when the envelope holds a
single footprint AND the user hasn't yet added one — until then
the chrome would just be noise.

## Source
Lines 142–246 in `crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pads_active_bar](/crates/oxide-app/src/library/editor/footprint/pads_active_bar.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
