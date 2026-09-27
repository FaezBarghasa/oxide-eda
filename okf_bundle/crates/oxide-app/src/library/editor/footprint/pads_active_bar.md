---
okf_version: "0.2"
type: Module
title: pads_active_bar
description: Pads-mode Active Bar — floating tool row for the footprint editor
resource: crates/oxide-app/src/library/editor/footprint/pads_active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pads_active_bar
language: rust
---

# pads_active_bar

Pads-mode Active Bar — floating tool row for the footprint editor

## Docstring

Pads-mode Active Bar — floating tool row for the footprint editor
when the editor is in [`EditorMode::Normal`].

Mirrors Altium's PCB Library editor active-bar layout: Select +
Place Pad / Track / Arc / String / Polygon / Hole, an Auto-fit
Courtyard toggle, and a Sketch-mode entry button. Tools that
aren't wired in v0.14.x ship as stubs (greyed icons, no
`on_press`) so the bar reads as the eventual finished surface,
not a half-built one.

The full set is intentional Altium parity — every pad/track/poly
placement lands as wiring goes in. The minimal v0.14.2 wiring is:
- Select: cursor (no specific message; the canvas's empty-space
click already adds a pad in this mode, so "Place Pad" stays
visually present in the bar but acts as a discoverability
reminder until a proper place-pad gesture lands).
- Auto-fit Courtyard: existing `FootprintToggleAutoFit` toggle.
- Edit Sketch: mode-switch to `EditorMode::Sketch`.

## Relationships

| Type | Target |
|------|--------|
| related | [mode_switcher_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/mode_switcher_overlay.md) |
| related | [footprint_tabs_overlay](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/footprint_tabs_overlay.md) |
| related | [items](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/items.md) |
| related | [view](/crates/oxide-app/src/library/editor/footprint/pads_active_bar/view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
