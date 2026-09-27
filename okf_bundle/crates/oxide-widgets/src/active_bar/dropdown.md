---
okf_version: "0.2"
type: Module
title: dropdown
description: "Generic active-bar dropdown widget — used by every editor's active"
resource: crates/oxide-widgets/src/active_bar/dropdown.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/active_bar/dropdown
language: rust
---

# dropdown

Generic active-bar dropdown widget — used by every editor's active

## Docstring

Generic active-bar dropdown widget — used by every editor's active
bar (schematic, footprint, future PCB) so chrome stays identical
across surfaces while each editor supplies its own actions.

## Contract for new editors (PCB integration target)

1. Define a `*_active_bar_menu: Option<MenuKind>` field on the
editor's state to track which dropdown is open.
2. Add a `ToggleActiveBarMenu(MenuKind)` message variant + a
`CloseActiveBarMenu` message; the dispatcher toggles / clears
the field.
3. Build a `dropdowns.rs` module exposing
`entries(menu, state, path, theme_id, ...) ->
Vec<DropdownEntry<EditorMessage>>` with one match arm per menu.
4. In the editor's active-bar `view` function, render the open
dropdown via `oxide_widgets::active_bar_dropdown::view(entries,
tokens, width_hint)` and stack it in a `Stack` overlay layer
above the bar with a transparent backstop layer for click-
outside-to-dismiss.

## What this widget renders

Driven by a `Vec<DropdownEntry<M>>`; the widget knows how to draw
section headers, separators, disabled rows, glyph + label rows,
and an optional checkmark / right-aligned shortcut hint. The
`Custom(Element<M>)` escape hatch lets editors compose chip-grid
layouts (the Selection Filter dropdown's pill grid) without
re-implementing the panel chrome.

## Helpers

- `chip_btn(label, on_press, enabled, accent) -> Element<M>` —
Altium-style toggle chip used inside `DropdownEntry::Custom` for
chip-grid layouts. Identical chrome across all editors.

## NOT included here

- The trigger button (`oxide_widgets::active_bar::ActiveBarButton`
handles that — left-click action + right-click dropdown +
chevron indicator).
- The toggle state (each editor's state owns it).
- The click-outside backstop layer (each editor's view stacks it).

## Relationships

| Type | Target |
|------|--------|
| related | [DropdownEntry](/crates/oxide-widgets/src/active_bar/dropdown/DropdownEntry.md) |
| related | [DropdownItem](/crates/oxide-widgets/src/active_bar/dropdown/DropdownItem.md) |
| related | [new](/crates/oxide-widgets/src/active_bar/dropdown/new.md) |
| related | [checked](/crates/oxide-widgets/src/active_bar/dropdown/checked.md) |
| related | [icon](/crates/oxide-widgets/src/active_bar/dropdown/icon.md) |
| related | [shortcut](/crates/oxide-widgets/src/active_bar/dropdown/shortcut.md) |
| related | [disabled](/crates/oxide-widgets/src/active_bar/dropdown/disabled.md) |
| related | [new](/crates/oxide-widgets/src/active_bar/dropdown/new.md) |
| related | [checked](/crates/oxide-widgets/src/active_bar/dropdown/checked.md) |
| related | [icon](/crates/oxide-widgets/src/active_bar/dropdown/icon.md) |
| related | [shortcut](/crates/oxide-widgets/src/active_bar/dropdown/shortcut.md) |
| related | [disabled](/crates/oxide-widgets/src/active_bar/dropdown/disabled.md) |
| related | [view](/crates/oxide-widgets/src/active_bar/dropdown/view.md) |
| related | [chip_btn](/crates/oxide-widgets/src/active_bar/dropdown/chip_btn.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
