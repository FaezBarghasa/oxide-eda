---
okf_version: "0.2"
type: Module
title: overlays
description: "Floating overlay builders — the symbol hover tooltip, the chrome-strip"
resource: crates/oxide-app/src/app/view/overlays/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/mod
language: rust
---

# overlays

Floating overlay builders — the symbol hover tooltip, the chrome-strip

## Docstring

Floating overlay builders — the symbol hover tooltip, the chrome-strip
command-palette dropdown, and the shared click-outside-to-dismiss
layer.

Extracted verbatim from `view/mod.rs` (ADR-0001, issue #164) as pure
code motion — no behaviour change. These are methods of the same
`Oxide` view impl, split across sibling files. The overlay-assembly
entry point `collect_overlays` stays in `view/mod.rs` alongside the
other composition-core methods.

## Relationships

| Type | Target |
|------|--------|
| related | [view_hover_tooltip](/crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip.md) |
| related | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
| related | [dismiss_layer](/crates/oxide-app/src/app/view/overlays/mod/dismiss_layer.md) |
| related | [active_bar_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/active_bar_menu_overlay.md) |
| related | [context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/context_menu_overlay.md) |
| related | [tab_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/tab_context_menu_overlay.md) |
| related | [project_tree_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/project_tree_context_menu_overlay.md) |
| related | [grid_picker_overlay](/crates/oxide-app/src/app/view/overlays/mod/grid_picker_overlay.md) |
| related | [dock_drag_zone_overlay](/crates/oxide-app/src/app/view/overlays/mod/dock_drag_zone_overlay.md) |
| related | [floating_panels_overlay](/crates/oxide-app/src/app/view/overlays/mod/floating_panels_overlay.md) |
| related | [view_hover_tooltip](/crates/oxide-app/src/app/view/overlays/mod/view_hover_tooltip.md) |
| related | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
| related | [dismiss_layer](/crates/oxide-app/src/app/view/overlays/mod/dismiss_layer.md) |
| related | [active_bar_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/active_bar_menu_overlay.md) |
| related | [context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/context_menu_overlay.md) |
| related | [tab_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/tab_context_menu_overlay.md) |
| related | [project_tree_context_menu_overlay](/crates/oxide-app/src/app/view/overlays/mod/project_tree_context_menu_overlay.md) |
| related | [grid_picker_overlay](/crates/oxide-app/src/app/view/overlays/mod/grid_picker_overlay.md) |
| related | [dock_drag_zone_overlay](/crates/oxide-app/src/app/view/overlays/mod/dock_drag_zone_overlay.md) |
| related | [floating_panels_overlay](/crates/oxide-app/src/app/view/overlays/mod/floating_panels_overlay.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
