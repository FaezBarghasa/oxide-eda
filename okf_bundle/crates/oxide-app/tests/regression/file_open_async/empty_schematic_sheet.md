---
okf_version: "0.2"
type: Function
title: empty_schematic_sheet
description: "Minimal but valid `SchematicSheet` fixture — same shape as"
resource: crates/oxide-app/tests/regression/file_open_async.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/file_open_async/empty_schematic_sheet
language: rust
---

# empty_schematic_sheet

Minimal but valid `SchematicSheet` fixture — same shape as

## Signature

```rust
fn empty_schematic_sheet() -> oxide_types::schematic::SchematicSheet
```

## Docstring

Minimal but valid `SchematicSheet` fixture — same shape as
`oxide_types::format::tests::empty_sheet`.

## Source
Lines 24–46 in `crates/oxide-app/tests/regression/file_open_async.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [file_open_async](/crates/oxide-app/tests/regression/file_open_async.md) |
| called_by | [opening_a_schematic_does_not_synchronously_create_a_tab](/crates/oxide-app/tests/regression/file_open_async/opening_a_schematic_does_not_synchronously_create_a_tab.md) |
| called_by | [opening_the_same_schematic_twice_before_it_completes_spawns_only_one_task](/crates/oxide-app/tests/regression/file_open_async/opening_the_same_schematic_twice_before_it_completes_spawns_only_one_task.md) |
| called_by | [reopening_an_already_open_schematic_tab_activates_it_instead_of_duplicating](/crates/oxide-app/tests/regression/file_open_async/reopening_an_already_open_schematic_tab_activates_it_instead_of_duplicating.md) |
| called_by | [schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did](/crates/oxide-app/tests/regression/file_open_async/schematic_open_finished_ok_opens_the_tab_like_the_old_sync_path_did.md) |
