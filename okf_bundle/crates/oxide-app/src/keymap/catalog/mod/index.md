# mod

## Classs

- [CommandFlags](CommandFlags.md) — GUI/undo/visibility flags for a command.
- [CommandGroup](CommandGroup.md) — Coarse editor-surface bucket used by the Keyboard Shortcuts pane to
- [CommandIdSnapshot](CommandIdSnapshot.md) — [derive(serde::Serialize)]
- [CommandMetadata](CommandMetadata.md) — [derive(Debug, Clone, Copy, PartialEq, Eq)]
- [DocumentKind](DocumentKind.md) — Coarse document-kind gate for [`Enablement::RequiresDocument`]. Mirrors
- [Enablement](Enablement.md) — Fixed predicate gating when a command is enabled. Evaluating this
- [IconId](IconId.md) — Surface-agnostic icon key. A view surface (menu, toolbar, command
- [KeyBind](KeyBind.md) — A command's suggested default keyboard shortcut, carried on its

## Functions

- [all_command_ids](all_command_ids.md) — Every command id in the catalog, in table order.
- [all_metadata](all_metadata.md) — Flattened iterator over every command's metadata, across all groups.
- [command_groups_follow_primary_surface](command_groups_follow_primary_surface.md) — [test]
- [command_id_surface_matches_golden_snapshot](command_id_surface_matches_golden_snapshot.md) — Golden-snapshot test (oxide#276): locks the STABLE command-id
- [display_name](display_name.md) — Human-readable header shown above each group.
- [display_name](display_name_1.md) — Human-readable header shown above each group.
- [every_catalog_entry_sets_enable_and_flags](every_catalog_entry_sets_enable_and_flags.md) — Every catalog row must state `enable` and `flags` explicitly.
- [every_command_has_a_group_in_display_order](every_command_has_a_group_in_display_order.md) — [test]
- [fallback_label](fallback_label.md)
- [group_of](group_of.md)
- [grouping_partitions_every_command_exactly_once](grouping_partitions_every_command_exactly_once.md) — [test]
- [icon_and_keybind_still_inherit_the_default](icon_and_keybind_still_inherit_the_default.md) — [test]
- [menu_label](menu_label.md) — The label a menu surface should display: the terse `menu_label`
- [menu_label](menu_label_1.md) — The label a menu surface should display: the terse `menu_label`
- [menu_label_falls_back_to_label_when_unset](menu_label_falls_back_to_label_when_unset.md) — [test]
- [menu_label_overrides_match_menu_bar_text](menu_label_overrides_match_menu_bar_text.md) — [test]
- [metadata_for](metadata_for.md)
