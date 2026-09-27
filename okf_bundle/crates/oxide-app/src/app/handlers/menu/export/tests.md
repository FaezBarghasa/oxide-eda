---
okf_version: "0.2"
type: Module
title: tests
description: "Export scope regressions (#406) — asserted on the *emitted page set*."
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests
language: rust
---

# tests

Export scope regressions (#406) — asserted on the *emitted page set*.

## Docstring

Export scope regressions (#406) — asserted on the *emitted page set*.

These drive [`super::build_export_scope`] itself rather than the scope
helper it calls, because the shipped artifact is the page list: the earlier
round of this fix tested only that a new helper returned what it was written
to return, which cannot go red on a revert.

The wrong-deliverable failures covered:

1. a loose schematic focused while a project is loaded must export *itself*,
not the sticky `active_project`'s sheet list;
2. a hierarchical child sheet — opened by descending into a sheet symbol,
which never adds it to the project's persisted `sheets` list — must
still resolve to its owning project and export the project's pages, not
ship a one-page PDF of the child alone;
3. …and the netlist that ships with it must not quietly omit that child's
subtree — the netlist input set is the child-sheet graph, not the
printed page set, so a `MissingChild` means *missing*, and when one is
genuine the `.net` export refuses while the PDF degrades loudly;
4. a project directory recorded in `data.dir` that no longer matches the
`.snxprj` on disk must not resolve ownership and sheet paths differently;
5. the loose page set is sorted, not `HashMap`-ordered.

## Relationships

| Type | Target |
|------|--------|
| related | [child_ref](/crates/oxide-app/src/app/handlers/menu/export/tests/child_ref.md) |
| related | [schematic](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic.md) |
| related | [sheet_with_net](/crates/oxide-app/src/app/handlers/menu/export/tests/sheet_with_net.md) |
| related | [net_names](/crates/oxide-app/src/app/handlers/menu/export/tests/net_names.md) |
| related | [netlist_references](/crates/oxide-app/src/app/handlers/menu/export/tests/netlist_references.md) |
| related | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| related | [workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/workspace.md) |
| related | [open](/crates/oxide-app/src/app/handlers/menu/export/tests/open.md) |
| related | [open_with](/crates/oxide-app/src/app/handlers/menu/export/tests/open_with.md) |
| related | [page_paths](/crates/oxide-app/src/app/handlers/menu/export/tests/page_paths.md) |
| related | [context](/crates/oxide-app/src/app/handlers/menu/export/tests/context.md) |
| related | [loose_schematic_exports_itself_not_the_sticky_projects_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_schematic_exports_itself_not_the_sticky_projects_sheets.md) |
| related | [hierarchical_child_sheet_exports_its_owning_project](/crates/oxide-app/src/app/handlers/menu/export/tests/hierarchical_child_sheet_exports_its_owning_project.md) |
| related | [root_active_netlist_contains_a_child_absent_from_data_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/root_active_netlist_contains_a_child_absent_from_data_sheets.md) |
| related | [a_child_only_on_disk_is_stitched_without_being_opened](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_only_on_disk_is_stitched_without_being_opened.md) |
| related | [a_project_export_roots_at_the_project_root_not_the_active_child](/crates/oxide-app/src/app/handlers/menu/export/tests/a_project_export_roots_at_the_project_root_not_the_active_child.md) |
| related | [grandchild_resolves_through_two_hops_to_the_project](/crates/oxide-app/src/app/handlers/menu/export/tests/grandchild_resolves_through_two_hops_to_the_project.md) |
| related | [listed_project_sheet_exports_the_whole_project](/crates/oxide-app/src/app/handlers/menu/export/tests/listed_project_sheet_exports_the_whole_project.md) |
| related | [schematic_inside_the_project_directory_but_unlisted_stays_loose](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic_inside_the_project_directory_but_unlisted_stays_loose.md) |
| related | [loose_export_page_order_is_stable_across_rebuilds](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_export_page_order_is_stable_across_rebuilds.md) |
| related | [a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths](/crates/oxide-app/src/app/handlers/menu/export/tests/a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths.md) |
| related | [diagnostic_count](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnostic_count.md) |
| related | [app_with_missing_child](/crates/oxide-app/src/app/handlers/menu/export/tests/app_with_missing_child.md) |
| related | [netlist_export_refuses_to_write_an_incomplete_netlist](/crates/oxide-app/src/app/handlers/menu/export/tests/netlist_export_refuses_to_write_an_incomplete_netlist.md) |
| related | [export_anyway_writes_a_partial_netlist_with_an_incomplete_header](/crates/oxide-app/src/app/handlers/menu/export/tests/export_anyway_writes_a_partial_netlist_with_an_incomplete_header.md) |
| related | [export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation](/crates/oxide-app/src/app/handlers/menu/export/tests/export_anyway_writes_from_the_prompt_snapshot_not_a_fresh_re_derivation.md) |
| related | [cancel_on_the_incomplete_prompt_writes_nothing](/crates/oxide-app/src/app/handlers/menu/export/tests/cancel_on_the_incomplete_prompt_writes_nothing.md) |
| related | [pdf_export_proceeds_and_warns_once_per_user_action](/crates/oxide-app/src/app/handlers/menu/export/tests/pdf_export_proceeds_and_warns_once_per_user_action.md) |
| related | [rerasterizing_the_preview_does_not_flood_the_messages_panel](/crates/oxide-app/src/app/handlers/menu/export/tests/rerasterizing_the_preview_does_not_flood_the_messages_panel.md) |
| related | [app_flat_project](/crates/oxide-app/src/app/handlers/menu/export/tests/app_flat_project.md) |
| related | [a_flat_projects_second_page_no_longer_vanishes_from_the_netlist](/crates/oxide-app/src/app/handlers/menu/export/tests/a_flat_projects_second_page_no_longer_vanishes_from_the_netlist.md) |
| related | [a_page_the_root_does_reach_is_not_reported_as_a_shortfall](/crates/oxide-app/src/app/handlers/menu/export/tests/a_page_the_root_does_reach_is_not_reported_as_a_shortfall.md) |
| related | [a_child_that_exists_but_will_not_parse_is_not_called_missing](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_that_exists_but_will_not_parse_is_not_called_missing.md) |
