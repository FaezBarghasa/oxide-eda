---
okf_version: "0.2"
type: Module
title: styles
description: "Custom Iced styles matching Altium Designer's dark theme chrome."
resource: crates/oxide-app/src/styles.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:37:26Z"
concept_id: crates/oxide-app/src/styles
language: rust
---

# styles

Custom Iced styles matching Altium Designer's dark theme chrome.

## Docstring

Custom Iced styles matching Altium Designer's dark theme chrome.

All style functions are token-aware factories that accept `&ThemeTokens`
and return closures. This ensures every UI component picks up theme
changes in real time.

## Relationships

| Type | Target |
|------|--------|
| related | [ti](/crates/oxide-app/src/styles/ti.md) |
| related | [panel_region](/crates/oxide-app/src/styles/panel_region.md) |
| related | [chrome_separator](/crates/oxide-app/src/styles/chrome_separator.md) |
| related | [toolbar_strip](/crates/oxide-app/src/styles/toolbar_strip.md) |
| related | [modal_header_strip](/crates/oxide-app/src/styles/modal_header_strip.md) |
| related | [modal_footer_strip](/crates/oxide-app/src/styles/modal_footer_strip.md) |
| related | [active_bar_strip](/crates/oxide-app/src/styles/active_bar_strip.md) |
| related | [status_bar](/crates/oxide-app/src/styles/status_bar.md) |
| related | [tab_bar_strip](/crates/oxide-app/src/styles/tab_bar_strip.md) |
| related | [collapsed_rail](/crates/oxide-app/src/styles/collapsed_rail.md) |
| related | [resize_handle](/crates/oxide-app/src/styles/resize_handle.md) |
| related | [panel_content](/crates/oxide-app/src/styles/panel_content.md) |
| related | [context_menu](/crates/oxide-app/src/styles/context_menu.md) |
| related | [modal_card](/crates/oxide-app/src/styles/modal_card.md) |
| related | [panel_card](/crates/oxide-app/src/styles/panel_card.md) |
| related | [floating_title_bar](/crates/oxide-app/src/styles/floating_title_bar.md) |
| related | [floating_panel_body](/crates/oxide-app/src/styles/floating_panel_body.md) |
| related | [floating_panel_shadow](/crates/oxide-app/src/styles/floating_panel_shadow.md) |
| related | [dock_zone_highlight](/crates/oxide-app/src/styles/dock_zone_highlight.md) |
| related | [dock_tab_container](/crates/oxide-app/src/styles/dock_tab_container.md) |
| related | [rail_tab](/crates/oxide-app/src/styles/rail_tab.md) |
| related | [menu_item](/crates/oxide-app/src/styles/menu_item.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
