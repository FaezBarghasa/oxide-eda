---
okf_version: "0.2"
type: Function
title: chrome_search_bar_geometry
description: "`(x, width)` of the chrome strip's search bar for a window `window_w`"
resource: crates/oxide-app/src/app/view/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/mod/chrome_search_bar_geometry
language: rust
---

# chrome_search_bar_geometry

`(x, width)` of the chrome strip's search bar for a window `window_w`

## Signature

```rust
pub(crate) fn chrome_search_bar_geometry(window_w: f32) -> (f32, f32)
```

## Visibility

- `pub(crate)`

## Docstring

`(x, width)` of the chrome strip's search bar for a window `window_w`
pixels wide. Single source of truth: the chrome strip lays the bar
out with this width, and the command-palette dropdown anchors to this
`x`, so the two can never drift apart.

The bar is centred on the **window**, not on the gap between the menu
row and the window controls — that gap is off-centre because the menu
row is much wider than the three control buttons. To stay centred
without overlapping either side, both sides reserve the same width:
whichever of the two is wider. Whatever is left over is the bar,
clamped to `[CHROME_SEARCH_BAR_MIN_WIDTH, CHROME_SEARCH_BAR_WIDTH]`,
so it shrinks as the window narrows and never grows past its design
width on a wide monitor.

## Source
Lines 61–71 in `crates/oxide-app/src/app/view/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [view](/crates/oxide-app/src/app/view/mod.md) |
| calls | [approx_menu_bar_width](/crates/oxide-app/src/menu_bar/mod/approx_menu_bar_width.md) |
| called_by | [view_main_window_chrome](/crates/oxide-app/src/app/view/chrome/view_main_window_chrome.md) |
| called_by | [search_bar_stays_centred_and_clamped](/crates/oxide-app/src/app/view/mod/search_bar_stays_centred_and_clamped.md) |
| called_by | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
