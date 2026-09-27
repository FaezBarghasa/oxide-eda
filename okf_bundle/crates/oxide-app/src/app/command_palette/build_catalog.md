---
okf_version: "0.2"
type: Function
title: build_catalog
description: "Build the full catalog from the live app state. Cheap: O(menu) +"
resource: crates/oxide-app/src/app/command_palette.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command_palette/build_catalog
language: rust
---

# build_catalog

Build the full catalog from the live app state. Cheap: O(menu) +

## Signature

```rust
pub fn build_catalog(app: &super::Oxide) -> Vec<CommandEntry>
```

## Visibility

- `pub`

## Docstring

Build the full catalog from the live app state. Cheap: O(menu) +
O(panels) + O(placed symbols across active project) +
O(sheets across all projects). Called once per query keystroke; the
catalog is filtered/scored downstream.

## Source
Lines 85–209 in `crates/oxide-app/src/app/command_palette.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command_palette](/crates/oxide-app/src/app/command_palette.md) |
| calls | [all_command_ids](/crates/oxide-app/src/keymap/catalog/mod/all_command_ids.md) |
| calls | [is_dispatchable](/crates/oxide-app/src/app/command/bridge/is_dispatchable.md) |
| calls | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
| calls | [Command](/crates/oxide-engine/src/command/Command.md) |
| called_by | [a_bound_command_row_shows_its_shortcut](/crates/oxide-app/src/app/command_palette/a_bound_command_row_shows_its_shortcut.md) |
| called_by | [every_palette_command_row_resolves_through_the_bridge](/crates/oxide-app/src/app/command_palette/every_palette_command_row_resolves_through_the_bridge.md) |
| called_by | [the_palette_offers_command_rows_from_the_catalog](/crates/oxide-app/src/app/command_palette/the_palette_offers_command_rows_from_the_catalog.md) |
| called_by | [execute_command_palette_selected](/crates/oxide-app/src/app/dispatch/command_palette/execute_command_palette_selected.md) |
| called_by | [move_command_palette_selection](/crates/oxide-app/src/app/dispatch/command_palette/move_command_palette_selection.md) |
| called_by | [view_command_palette_dropdown](/crates/oxide-app/src/app/view/overlays/mod/view_command_palette_dropdown.md) |
