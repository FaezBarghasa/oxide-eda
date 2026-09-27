---
okf_version: "0.2"
type: Module
title: file_open_async
description: "#99 — schematic/PCB file-open read+parse is async."
resource: crates/oxide-app/tests/regression/file_open_async.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/file_open_async
language: rust
---

# file_open_async

#99 — schematic/PCB file-open read+parse is async.

## Docstring

#99 — schematic/PCB file-open read+parse is async.

`open_schematic_file` / `open_pcb_file` used to `fs::read_to_string`
+ parse synchronously inside `update()`. They now return a
`Task::perform` (`spawn_blocking` body) that completes with
`FileMsg::SchematicOpenFinished` / `FileMsg::PcbOpenFinished` —
mirrors the `HistoryLoaded` pattern. These tests pin both halves:
`update()` must not synchronously open the tab, and the completion
message must apply exactly like the old inline path did (success
opens the tab, failure logs and opens nothing).

## Relationships

| Type | Target |
|------|--------|
| related | [empty_schematic_sheet](/crates/oxide-app/tests/regression/file_open_async/empty_schematic_sheet.md) |
| related | [opening_a_schematic_does_not_synchronously_create_a_tab](/crates/oxide-app/tests/regression/file_open_async/opening_a_schematic_does_not_synchronously_create_a_tab.md) |
| related | [opening_the_same_schematic_twice_before_it_completes_spawns_only_one_task](/crates/oxide-app/tests/regression/file_open_async/opening_the_same_schematic_twice_before_it_completes_spawns_only_one_task.md) |
| related | [schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
| related | [schematic_open_finished_err_opens_no_tab_and_does_not_panic](/crates/oxide-app/tests/regression/file_open_async/schematic_open_finished_err_opens_no_tab_and_does_not_panic.md) |
| related | [reopening_an_already_open_schematic_tab_activates_it_instead_of_duplicating](/crates/oxide-app/tests/regression/file_open_async/reopening_an_already_open_schematic_tab_activates_it_instead_of_duplicating.md) |
| related | [empty_pcb_board](/crates/oxide-app/tests/regression/file_open_async/empty_pcb_board.md) |
| related | [opening_the_same_pcb_twice_before_it_completes_spawns_only_one_task](/crates/oxide-app/tests/regression/file_open_async/opening_the_same_pcb_twice_before_it_completes_spawns_only_one_task.md) |
| related | [pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/pcb_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
| related | [pcb_open_finished_err_opens_no_tab_and_does_not_panic](/crates/oxide-app/tests/regression/file_open_async/pcb_open_finished_err_opens_no_tab_and_does_not_panic.md) |
