---
okf_version: "0.2"
type: Function
title: to_color
description: "Convert a oxide `Color` (u8 components) to an Iced `Color` (f32 0..1)."
resource: crates/oxide-widgets/src/theme_ext.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/theme_ext/to_color
language: rust
---

# to_color

Convert a oxide `Color` (u8 components) to an Iced `Color` (f32 0..1).

## Signature

```rust
pub fn to_color(c: &SxColor) -> Color
```

## Visibility

- `pub`

## Docstring

Convert a oxide `Color` (u8 components) to an Iced `Color` (f32 0..1).

## Source
Lines 15–17 in `crates/oxide-widgets/src/theme_ext.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [theme_ext](/crates/oxide-widgets/src/theme_ext.md) |
| called_by | [view](/crates/oxide-widgets/src/active_bar/dropdown/view.md) |
| called_by | [view](/crates/oxide-widgets/src/active_bar/mod/view.md) |
| called_by | [view_button](/crates/oxide-widgets/src/active_bar/mod/view_button.md) |
| called_by | [status_bar](/crates/oxide-widgets/src/status_bar/status_bar.md) |
| called_by | [accent](/crates/oxide-widgets/src/theme_ext/accent.md) |
| called_by | [accent_bg](/crates/oxide-widgets/src/theme_ext/accent_bg.md) |
| called_by | [accent_color](/crates/oxide-widgets/src/theme_ext/accent_color.md) |
| called_by | [app_bg](/crates/oxide-widgets/src/theme_ext/app_bg.md) |
| called_by | [border_color](/crates/oxide-widgets/src/theme_ext/border_color.md) |
| called_by | [error_color](/crates/oxide-widgets/src/theme_ext/error_color.md) |
| called_by | [hover_color](/crates/oxide-widgets/src/theme_ext/hover_color.md) |
| called_by | [panel_bg](/crates/oxide-widgets/src/theme_ext/panel_bg.md) |
| called_by | [paper_bg](/crates/oxide-widgets/src/theme_ext/paper_bg.md) |
| called_by | [selection_color](/crates/oxide-widgets/src/theme_ext/selection_color.md) |
| called_by | [status_bar_bg](/crates/oxide-widgets/src/theme_ext/status_bar_bg.md) |
| called_by | [success_color](/crates/oxide-widgets/src/theme_ext/success_color.md) |
| called_by | [text_primary](/crates/oxide-widgets/src/theme_ext/text_primary.md) |
| called_by | [text_secondary](/crates/oxide-widgets/src/theme_ext/text_secondary.md) |
| called_by | [toolbar_bg](/crates/oxide-widgets/src/theme_ext/toolbar_bg.md) |
| called_by | [warning_color](/crates/oxide-widgets/src/theme_ext/warning_color.md) |
