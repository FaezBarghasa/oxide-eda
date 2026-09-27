---
okf_version: "0.2"
type: Module
title: menu_command_tests
description: "Drift guard (#272): every `AppCommandId` a menu surface names must"
resource: crates/oxide-app/src/keymap/menu_command_tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/menu_command_tests
language: rust
---

# menu_command_tests

Drift guard (#272): every `AppCommandId` a menu surface names must

## Docstring

Drift guard (#272): every `AppCommandId` a menu surface names must
resolve in the command catalog (`CommandMetadata`). The menu surfaces
reference commands by string id — `menu_bar` via `shortcut_for` /
`cmd_label`, the context menu via `keymap_shortcut_label` — and derive
the row's shortcut (and, since #282, its label) from that id. If a menu
adds a row keyed on an id with no catalog entry, the shortcut/label
lookup silently falls back to the literal and the command never reaches
the Keyboard Shortcuts pane / palette. This test scans the menu source
files and fails on any such orphan id, so the catalog stays the single
source of truth as menus evolve.

Source-scanning keeps the guard automatic: a new menu row written with
any of the tracked call shapes is checked without editing this test.
`active_bar` is intentionally out of scope — its dropdown rows dispatch
`ActiveBarAction` enum variants, not `AppCommandId` strings, so there is
no id to drift.

## Relationships

| Type | Target |
|------|--------|
| related | [ids_from_call](/crates/oxide-app/src/keymap/menu_command_tests/ids_from_call.md) |
| related | [every_menu_command_id_resolves_in_the_catalog](/crates/oxide-app/src/keymap/menu_command_tests/every_menu_command_id_resolves_in_the_catalog.md) |
