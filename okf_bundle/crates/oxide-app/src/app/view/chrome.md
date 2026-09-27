---
okf_version: "0.2"
type: Module
title: chrome
description: Window chrome and layout scaffolding — the borderless main-window
resource: crates/oxide-app/src/app/view/chrome.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/chrome
language: rust
---

# chrome

Window chrome and layout scaffolding — the borderless main-window

## Docstring

Window chrome and layout scaffolding — the borderless main-window
chrome strip, the Preferences body, the detached-modal frame, and the
dock-panel / resize-handle helpers.

Extracted verbatim from `view/mod.rs` (ADR-0001, issue #164) as pure
code motion — no behaviour change. These are methods of the same
`Oxide` view impl, split across sibling files.

## Relationships

| Type | Target |
|------|--------|
| related | [view_main_window_chrome](/crates/oxide-app/src/app/view/chrome/view_main_window_chrome.md) |
| related | [view_preferences_body](/crates/oxide-app/src/app/view/chrome/view_preferences_body.md) |
| related | [view_detached_modal](/crates/oxide-app/src/app/view/chrome/view_detached_modal.md) |
| related | [detached_modal_resize_overlay](/crates/oxide-app/src/app/view/chrome/detached_modal_resize_overlay.md) |
| related | [resize_edges_overlay](/crates/oxide-app/src/app/view/chrome/resize_edges_overlay.md) |
| related | [view_dock_panel](/crates/oxide-app/src/app/view/chrome/view_dock_panel.md) |
| related | [view_dock_panel_h](/crates/oxide-app/src/app/view/chrome/view_dock_panel_h.md) |
| related | [view_resize_handle](/crates/oxide-app/src/app/view/chrome/view_resize_handle.md) |
| related | [view_main_window_chrome](/crates/oxide-app/src/app/view/chrome/view_main_window_chrome.md) |
| related | [view_preferences_body](/crates/oxide-app/src/app/view/chrome/view_preferences_body.md) |
| related | [view_detached_modal](/crates/oxide-app/src/app/view/chrome/view_detached_modal.md) |
| related | [detached_modal_resize_overlay](/crates/oxide-app/src/app/view/chrome/detached_modal_resize_overlay.md) |
| related | [resize_edges_overlay](/crates/oxide-app/src/app/view/chrome/resize_edges_overlay.md) |
| related | [view_dock_panel](/crates/oxide-app/src/app/view/chrome/view_dock_panel.md) |
| related | [view_dock_panel_h](/crates/oxide-app/src/app/view/chrome/view_dock_panel_h.md) |
| related | [view_resize_handle](/crates/oxide-app/src/app/view/chrome/view_resize_handle.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
