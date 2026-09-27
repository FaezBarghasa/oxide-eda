---
okf_version: "0.2"
type: Function
title: view_footprint_canvas
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/view_footprint_canvas
language: rust
---

# view_footprint_canvas

## Signature

```rust
fn view_footprint_canvas(
    editor: &'a FootprintEditorState,
    tokens: &'a ThemeTokens,
    bg: iced::Color,
    grid: iced::Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 613–664 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| calls | [translate_footprint_canvas_msg](/crates/oxide-app/src/library/editor/standalone/footprint/translate_footprint_canvas_msg.md) |
| called_by | [view_footprint](/crates/oxide-app/src/library/editor/standalone/footprint/view_footprint.md) |
