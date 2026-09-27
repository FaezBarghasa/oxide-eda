---
okf_version: "0.2"
type: Module
title: bridge
description: "The app's single id→[`Message`] bridge."
resource: crates/oxide-app/src/app/command/bridge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/command/bridge
language: rust
---

# bridge

The app's single id→[`Message`] bridge.

## Docstring

The app's single id→[`Message`] bridge.

[`core_to_message`] turns a stable `AppCommandId` (the id a profile
TOML binds a key to) into the app's namespaced [`Message`] tree
(ADR-0001 D3). It is not keymap-specific — menus, the command
palette, and a future CLI all route a command id through here on
their way to `update`.

## Relationships

| Type | Target |
|------|--------|
| related | [core_to_message](/crates/oxide-app/src/app/command/bridge/core_to_message.md) |
| related | [is_dispatchable](/crates/oxide-app/src/app/command/bridge/is_dispatchable.md) |
| related | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
| related | [log_unmapped](/crates/oxide-app/src/app/command/bridge/log_unmapped.md) |
| related | [match_block](/crates/oxide-app/src/app/command/bridge/match_block.md) |
| related | [resolvable_command_ids](/crates/oxide-app/src/app/command/bridge/resolvable_command_ids.md) |
| related | [bridged_command_ids](/crates/oxide-app/src/app/command/bridge/bridged_command_ids.md) |
| related | [every_bridged_command_id_resolves_in_the_catalog](/crates/oxide-app/src/app/command/bridge/every_bridged_command_id_resolves_in_the_catalog.md) |
| related | [unmapped_command_ids_only_shrink](/crates/oxide-app/src/app/command/bridge/unmapped_command_ids_only_shrink.md) |
| related | [the_newly_wired_ids_reach_their_named_messages](/crates/oxide-app/src/app/command/bridge/the_newly_wired_ids_reach_their_named_messages.md) |
| related | [resolve](/crates/oxide-app/src/app/command/bridge/resolve.md) |
| related | [pinned_unmapped_ids_still_exist_in_the_catalog](/crates/oxide-app/src/app/command/bridge/pinned_unmapped_ids_still_exist_in_the_catalog.md) |
| related | [serial](/crates/oxide-app/src/app/command/bridge/serial.md) |
| related | [records_naming](/crates/oxide-app/src/app/command/bridge/records_naming.md) |
| related | [asking_whether_a_command_resolves_reports_nothing](/crates/oxide-app/src/app/command/bridge/asking_whether_a_command_resolves_reports_nothing.md) |
| related | [dispatching_an_unmapped_command_still_reports_it](/crates/oxide-app/src/app/command/bridge/dispatching_an_unmapped_command_still_reports_it.md) |
