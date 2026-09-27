---
okf_version: "0.2"
type: Module
title: context_menu
description: v0.26 — Right-click canvas context menu for the footprint editor.
resource: crates/oxide-app/src/library/editor/footprint/context_menu.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/context_menu
language: rust
---

# context_menu

v0.26 — Right-click canvas context menu for the footprint editor.

## Docstring

v0.26 — Right-click canvas context menu for the footprint editor.

Mirrors Altium's PCB Library Editor right-click conventions:
- On bare canvas: Place ▸ / View ▸ / Selection ▸ / Properties /
Find Similar.
- On a pad: Properties / Pad Actions ▸ stub / Cut / Copy / Delete /
Find Similar.
- On a silk graphic: Properties / Cut / Copy / Delete / Find Similar.

Mounting site: the layer-site loop at `app/view/mod.rs::collect_overlays`
pushes `view_context_menu(...)` immediately after the active bar
overlay so the dismiss layer occludes everything else.

Coords: `editor.state.context_menu.x/y` are window-absolute screen
coords (computed in `canvas.rs::ButtonReleased(Right)` from
`bounds.x + cursor.x`). The Translate widget at the call site
positions the card at exactly those pixels.

## Relationships

| Type | Target |
|------|--------|
| related | [silk_kind_label](/crates/oxide-app/src/library/editor/footprint/context_menu/silk_kind_label.md) |
| related | [view_context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu/view_context_menu.md) |
| related | [shortcut_label](/crates/oxide-app/src/library/editor/footprint/context_menu/shortcut_label.md) |
| related | [item_msg](/crates/oxide-app/src/library/editor/footprint/context_menu/item_msg.md) |
| related | [item_indented](/crates/oxide-app/src/library/editor/footprint/context_menu/item_indented.md) |
| related | [item_disabled](/crates/oxide-app/src/library/editor/footprint/context_menu/item_disabled.md) |
| related | [item_submenu_header](/crates/oxide-app/src/library/editor/footprint/context_menu/item_submenu_header.md) |
| related | [separator](/crates/oxide-app/src/library/editor/footprint/context_menu/separator.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
