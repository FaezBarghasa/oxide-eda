---
okf_version: "0.2"
type: Function
title: mode_switcher_overlay
description: v0.14.2 — standalone floating mode-switch widget rendered at the
resource: crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pads_active_bar/mode_switcher_overlay
language: rust
---

# mode_switcher_overlay

v0.14.2 — standalone floating mode-switch widget rendered at the

## Signature

```rust
pub fn mode_switcher_overlay(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> iced::Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

v0.14.2 — standalone floating mode-switch widget rendered at the
top-left of the canvas via `Stack` overlay (separate from the
active bar's tools). Three connected segments in **Sketch /
Pads / 3D** order; the active segment paints with the accent
background.

## Source
Lines 38–134 in `crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pads_active_bar](/crates/oxide-app/src/library/editor/footprint/pads_active_bar.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [view_footprint](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint.md) |
