---
okf_version: "0.2"
type: Module
title: unified_active_bar
description: v0.18.14 — Altium-style unified active bar for the footprint
resource: crates/oxide-app/src/library/editor/footprint/unified_active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/unified_active_bar
language: rust
---

# unified_active_bar

v0.18.14 — Altium-style unified active bar for the footprint

## Docstring

v0.18.14 — Altium-style unified active bar for the footprint
editor. Replaces the per-mode `pads_active_bar::view` /
`sketch_mode::active_bar::view` mounting in `standalone.rs`.

v0.13 — Public surface split into `bar_items()` + `dropdown_overlay()`
so the layer-site mounting code at `view_main_for` calls
`oxide_widgets::active_bar::view(items, tokens).map(...)` directly,
BYTE-FOR-BYTE matching the schematic active bar's chain. This
prevents `Element::map` ordering drift (Map-wraps-container vs
container-wraps-Map) that introduced a 2 px layout-pass shift.

## Relationships

| Type | Target |
|------|--------|
| related | [bar_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_items.md) |
| related | [menu_trigger_geometry](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry.md) |
| related | [dropdown_overlay](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_overlay.md) |
| related | [dropdown_trigger_items](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/dropdown_trigger_items.md) |
| related | [editor_in](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/editor_in.md) |
| related | [menu_triggers_are_located_by_message_not_by_index](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_triggers_are_located_by_message_not_by_index.md) |
| related | [bar_width_counts_every_slot_including_the_custom_one](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/bar_width_counts_every_slot_including_the_custom_one.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
