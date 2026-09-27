---
okf_version: "0.2"
type: Function
title: metadata_for
resource: crates/oxide-app/src/keymap/catalog/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/catalog/mod/metadata_for
language: rust
---

# metadata_for

## Signature

```rust
pub fn metadata_for(command: &AppCommandId) -> Option<&'static CommandMetadata>
```

## Visibility

- `pub`

## Source
Lines 233–235 in `crates/oxide-app/src/keymap/catalog/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [catalog](/crates/oxide-app/src/keymap/catalog/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [all_metadata](/crates/oxide-app/src/keymap/catalog/mod/all_metadata.md) |
| called_by | [action_label](/crates/oxide-app/src/app/command/active_bar/action_label.md) |
| called_by | [catalog_labels_match_the_active_bar_literals](/crates/oxide-app/src/app/command/active_bar/catalog_labels_match_the_active_bar_literals.md) |
| called_by | [every_active_bar_command_id_is_in_the_catalog](/crates/oxide-app/src/app/command/active_bar/every_active_bar_command_id_is_in_the_catalog.md) |
| called_by | [every_bridged_command_id_resolves_in_the_catalog](/crates/oxide-app/src/app/command/bridge/every_bridged_command_id_resolves_in_the_catalog.md) |
| called_by | [log_unmapped](/crates/oxide-app/src/app/command/bridge/log_unmapped.md) |
| called_by | [build_catalog](/crates/oxide-app/src/app/command_palette/build_catalog.md) |
| called_by | [menu_label_falls_back_to_label_when_unset](/crates/oxide-app/src/keymap/catalog/mod/menu_label_falls_back_to_label_when_unset.md) |
| called_by | [menu_label_overrides_match_menu_bar_text](/crates/oxide-app/src/keymap/catalog/mod/menu_label_overrides_match_menu_bar_text.md) |
| called_by | [every_menu_command_id_resolves_in_the_catalog](/crates/oxide-app/src/keymap/menu_command_tests/every_menu_command_id_resolves_in_the_catalog.md) |
| called_by | [cmd_label](/crates/oxide-app/src/menu_bar/mod/cmd_label.md) |
| called_by | [render](/crates/oxide-app/tests/command_reference/render.md) |
