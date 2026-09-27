---
okf_version: "0.2"
type: Function
title: view_footprint_layers_strip
description: v0.14.2 — Altium PCB-Library-style layer tab strip at the bottom
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_layers_strip
language: rust
---

# view_footprint_layers_strip

v0.14.2 — Altium PCB-Library-style layer tab strip at the bottom

## Signature

```rust
fn view_footprint_layers_strip(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Docstring

v0.14.2 — Altium PCB-Library-style layer tab strip at the bottom
of the canvas (above the footer). Each layer is a clickable pill
with a colour swatch + label. Click toggles visibility (existing
`FootprintToggleLayer` message).

Replaces the heavy layer pills that used to sit at the top of the
editor; moving them below the canvas keeps the top compact and
matches Altium's bottom-of-canvas layer tab pattern.

## Source
Lines 252–332 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [text_primary](/crates/oxide-app/src/preferences/widgets/text_primary.md) |
| calls | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
| called_by | [view_footprint](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint.md) |
