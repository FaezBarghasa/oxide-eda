---
okf_version: "0.2"
type: Function
title: all_command_ids
description: "Every command id in the catalog, in table order."
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/all_command_ids
language: rust
---

# all_command_ids

Every command id in the catalog, in table order.

## Signature

```rust
pub fn all_command_ids() -> impl Iterator<Item = &'static str>
```

## Visibility

- `pub`

## Docstring

Every command id in the catalog, in table order.

Exists so a consumer can ask "what is in the catalog?" without
needing `CommandMetadata` itself — the bridge's coverage ratchet
(`app::command::bridge`) compares this against the ids
`core_to_message` actually matches.

## Source
Lines 229–231 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
| calls | [all_metadata](/crates/oxide-app/src/keymap/catalog/mod/all_metadata.md) |
| called_by | [pinned_unmapped_ids_still_exist_in_the_catalog](/crates/oxide-app/src/app/command/bridge/pinned_unmapped_ids_still_exist_in_the_catalog.md) |
| called_by | [unmapped_command_ids_only_shrink](/crates/oxide-app/src/app/command/bridge/unmapped_command_ids_only_shrink.md) |
| called_by | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| called_by | [render](/crates/oxide-app/tests/command_reference/render.md) |
