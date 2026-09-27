---
okf_version: "0.2"
type: Function
title: dropdown_overlay
description: Build the dropdown overlay (panel + click-outside backstop) for
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_overlay
language: rust
---

# dropdown_overlay

Build the dropdown overlay (panel + click-outside backstop) for

## Signature

```rust
pub fn dropdown_overlay(
    editor: &'a FootprintEditorState,
    theme_id: ThemeId,
    tokens: &'a ThemeTokens,
    footprint_filter_presets: &[crate::active_bar::FootprintFilterPreset],
    top_padding_px: u16,
    window_width: f32,
) -> Option<iced::Element<'a, LibraryMessage>>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Build the dropdown overlay (panel + click-outside backstop) for
the currently-open menu. Returns `None` when no menu is open.
Caller pushes the result as a separate layer above the bar.

v0.26-H — dropdown is now Translate-positioned so it lands
directly under the trigger button instead of being centered
in the viewport.

`top_padding_px` is the y-offset (from the overlay's top edge,
which sits at window y=0) where the dropdown panel should land —
callers compute it as `y_offset + 4 + bar_height + small_gap` so
the panel touches the bar's bottom edge regardless of whether the
tab strip is showing.

`window_width` is the current window's pixel width — needed to
compute the bar's left edge when iced centre-aligns it.

## Source
Lines 99–171 in `crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unified_active_bar](/crates/oxide-app/src/library/editor/footprint/unified_active_bar.md) |
| calls | [menu_trigger_geometry](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry.md) |
| calls | [bar_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items.md) |
