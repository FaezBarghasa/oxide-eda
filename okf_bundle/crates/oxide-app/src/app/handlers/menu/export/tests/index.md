# tests

## Subdirectories

- [diagnosis](diagnosis/index.md)

## Modules

- [diagnosis](diagnosis.md) — How the export diagnoses a listed page it could not put in the netlist.

## Functions

- [a_child_only_on_disk_is_stitched_without_being_opened](a_child_only_on_disk_is_stitched_without_being_opened.md) — [test]
- [a_child_that_exists_but_will_not_parse_is_not_called_missing](a_child_that_exists_but_will_not_parse_is_not_called_missing.md) — [test]
- [a_flat_projects_second_page_no_longer_vanishes_from_the_netlist](a_flat_projects_second_page_no_longer_vanishes_from_the_netlist.md) — [test]
- [a_page_the_root_does_reach_is_not_reported_as_a_shortfall](a_page_the_root_does_reach_is_not_reported_as_a_shortfall.md) — [test]
- [a_project_export_roots_at_the_project_root_not_the_active_child](a_project_export_roots_at_the_project_root_not_the_active_child.md) — [test]
- [a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths](a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths.md) — [test]
- [app_flat_project](app_flat_project.md) — A flat two-page project: both pages are listed, neither references the
- [app_with_missing_child](app_with_missing_child.md) — A project whose root references `missing` — a child that is neither open
- [app_workspace](app_workspace.md) — A `Oxide` with one loaded, *active* project whose persisted sheet list is
- [cancel_on_the_incomplete_prompt_writes_nothing](cancel_on_the_incomplete_prompt_writes_nothing.md) — [test]
- [child_ref](child_ref.md)
- [context](context.md) — The context alone, for the cases that do not care about stitch issues.
- [diagnostic_count](diagnostic_count.md) — Count the diagnostics mentioning `marker`. The panel buffer is global, so
- [export_anyway_writes_a_partial_netlist_with_an_incomplete_header](export_anyway_writes_a_partial_netlist_with_an_incomplete_header.md) — [test]
- [export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation](export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation.md) — [test]
- [grandchild_resolves_through_two_hops_to_the_project](grandchild_resolves_through_two_hops_to_the_project.md) — [test]
- [hierarchical_child_sheet_exports_its_owning_project](hierarchical_child_sheet_exports_its_owning_project.md) — [test]
- [listed_project_sheet_exports_the_whole_project](listed_project_sheet_exports_the_whole_project.md) — [test]
- [loose_export_page_order_is_stable_across_rebuilds](loose_export_page_order_is_stable_across_rebuilds.md) — [test]
- [loose_schematic_exports_itself_not_the_sticky_projects_sheets](loose_schematic_exports_itself_not_the_sticky_projects_sheets.md) — [test]
- [net_names](net_names.md)
- [netlist_export_refuses_to_write_an_incomplete_netlist](netlist_export_refuses_to_write_an_incomplete_netlist.md) — [test]
- [netlist_references](netlist_references.md) — Every component reference the exported netlist carries a terminal for —
- [open](open.md) — Open `path` as a live engine whose sheet references `children`.
- [open_with](open_with.md)
- [page_paths](page_paths.md)
- [pdf_export_proceeds_and_warns_once_per_user_action](pdf_export_proceeds_and_warns_once_per_user_action.md) — [test]
- [rerasterizing_the_preview_does_not_flood_the_messages_panel](rerasterizing_the_preview_does_not_flood_the_messages_panel.md) — [test]
- [root_active_netlist_contains_a_child_absent_from_data_sheets](root_active_netlist_contains_a_child_absent_from_data_sheets.md) — [test]
- [schematic](schematic.md)
- [schematic_inside_the_project_directory_but_unlisted_stays_loose](schematic_inside_the_project_directory_but_unlisted_stays_loose.md) — [test]
- [sheet_with_net](sheet_with_net.md) — A sheet with one component pin sitting on a wire named by a `Global` label.
- [workspace](workspace.md)
