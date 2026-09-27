---
okf_version: "0.2"
type: Function
title: view_main_window_chrome
description: Custom chrome for the borderless main window. Replaces the OS
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome/view_main_window_chrome_1
language: rust
---

# view_main_window_chrome

Custom chrome for the borderless main window. Replaces the OS

## Signature

```rust
pub(super) fn view_main_window_chrome(
        &self,
        menu_row: Element<'a, Message>,
        tokens: &oxide_types::theme::ThemeTokens,
    ) -> Element<'a, Message>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Custom chrome for the borderless main window. Replaces the OS
title bar with a 36 px strip:

`[wordmark + menus] [drag] [drag] [min│max│×]`

with the search bar stacked on top of it, centred on the window
(see `chrome_search_bar_geometry`).

The drag zones are the only mouse-area clickable regions — menu
buttons, search, and window controls keep their own click
handlers. Double-click on a drag zone toggles maximize.

## Source
Lines 23–238 in `crates/oxide-app/src/app/view/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/app/view/chrome.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
| calls | [chrome_search_bar_geometry](/crates/oxide-app/src/app/view/mod/chrome_search_bar_geometry.md) |
| calls | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
