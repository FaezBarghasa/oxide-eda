---
okf_version: "0.2"
type: Function
title: all_metadata
description: "Flattened iterator over every command's metadata, across all groups."
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/all_metadata
language: rust
---

# all_metadata

Flattened iterator over every command's metadata, across all groups.

## Signature

```rust
fn all_metadata() -> impl Iterator<Item = &'static CommandMetadata>
```

## Docstring

Flattened iterator over every command's metadata, across all groups.

## Source
Lines 219–221 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
| called_by | [all_command_ids](/crates/oxide-app/src/keymap/catalog/mod/all_command_ids.md) |
| called_by | [command_id_surface_matches_golden_snapshot](/crates/oxide-app/src/keymap/catalog/mod/command_id_surface_matches_golden_snapshot.md) |
| called_by | [every_command_has_a_group_in_display_order](/crates/oxide-app/src/keymap/catalog/mod/every_command_has_a_group_in_display_order.md) |
| called_by | [group_of](/crates/oxide-app/src/keymap/catalog/mod/group_of.md) |
| called_by | [grouping_partitions_every_command_exactly_once](/crates/oxide-app/src/keymap/catalog/mod/grouping_partitions_every_command_exactly_once.md) |
| called_by | [icon_and_keybind_still_inherit_the_default](/crates/oxide-app/src/keymap/catalog/mod/icon_and_keybind_still_inherit_the_default.md) |
| called_by | [metadata_for](/crates/oxide-app/src/keymap/catalog/mod/metadata_for.md) |
