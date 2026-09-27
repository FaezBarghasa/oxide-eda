---
okf_version: "0.2"
type: Module
title: active_bar
description: "Stable command ids for the schematic Active Bar's actions."
resource: crates/oxide-app/src/app/command/active_bar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/command/active_bar
language: rust
---

# active_bar

Stable command ids for the schematic Active Bar's actions.

## Docstring

Stable command ids for the schematic Active Bar's actions.

One table rather than 59 match arms in [`super::bridge`], so an id and
the action it names sit on the same line, and a test can check the
pairing in both directions.

**Not every `ActiveBarAction` is here.** The parameterised families are
deliberately absent — each enumerates a value domain and wants one
command plus an argument rather than N frozen ids, which needs the
`CommandArgs` boundary (#367 / #366):
- `ActiveBarAction::InsideArea`
- `ActiveBarAction::OutsideArea`
- `ActiveBarAction::TouchingRectangle`
- `ActiveBarAction::TouchingLine`
- `ActiveBarAction::PlacePowerGND`
- `ActiveBarAction::PlacePowerVCC`
- `ActiveBarAction::PlacePowerPlus12`
- `ActiveBarAction::PlacePowerPlus5`
- `ActiveBarAction::PlacePowerMinus5`
- `ActiveBarAction::PlacePowerArrow`
- `ActiveBarAction::PlacePowerWave`
- `ActiveBarAction::PlacePowerBar`
- `ActiveBarAction::PlacePowerCircle`
- `ActiveBarAction::PlacePowerSignalGND`
- `ActiveBarAction::PlacePowerEarth`
- `ActiveBarAction::NetColorBlue`
- `ActiveBarAction::NetColorLightGreen`
- `ActiveBarAction::NetColorLightBlue`
- `ActiveBarAction::NetColorRed`
- `ActiveBarAction::NetColorFuchsia`
- `ActiveBarAction::NetColorYellow`
- `ActiveBarAction::NetColorDarkGreen`
- `ActiveBarAction::NetColorCustom`

Two more are absent because [`super::bridge`] already maps their ids to
a different message: `select_all` goes through `SelectionRequest` and
`place_net_label` through the tool picker. Listing them here would give
one id two meanings.

See `docs/audit/command-registry-action-surface-2026-07-25.md` §4.

## Relationships

| Type | Target |
|------|--------|
| related | [action_for_id](/crates/oxide-app/src/app/command/active_bar/action_for_id.md) |
| related | [id_for_action](/crates/oxide-app/src/app/command/active_bar/id_for_action.md) |
| related | [variant_name](/crates/oxide-app/src/app/command/active_bar/variant_name.md) |
| related | [action_label](/crates/oxide-app/src/app/command/active_bar/action_label.md) |
| related | [command_ids](/crates/oxide-app/src/app/command/active_bar/command_ids.md) |
| related | [every_active_bar_command_id_is_in_the_catalog](/crates/oxide-app/src/app/command/active_bar/every_active_bar_command_id_is_in_the_catalog.md) |
| related | [the_table_has_no_duplicates](/crates/oxide-app/src/app/command/active_bar/the_table_has_no_duplicates.md) |
| related | [action_lookup_round_trips](/crates/oxide-app/src/app/command/active_bar/action_lookup_round_trips.md) |
| related | [catalog_labels_match_the_active_bar_literals](/crates/oxide-app/src/app/command/active_bar/catalog_labels_match_the_active_bar_literals.md) |
| related | [scan_dropdown_rows](/crates/oxide-app/src/app/command/active_bar/scan_dropdown_rows.md) |
| related | [every_active_bar_action_is_accounted_for](/crates/oxide-app/src/app/command/active_bar/every_active_bar_action_is_accounted_for.md) |
