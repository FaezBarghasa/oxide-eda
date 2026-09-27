---
okf_version: "0.2"
type: Function
title: workspace
resource: crates/oxide-app/src/app/handlers/menu/export/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:50:59Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/tests/workspace
language: rust
---

# workspace

## Signature

```rust
fn workspace(dir: &str, listed: &[&str]) -> DocumentState
```

## Source
Lines 233–235 in `crates/oxide-app/src/app/handlers/menu/export/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/app/handlers/menu/export/tests.md) |
| calls | [app_workspace](/crates/oxide-app/src/app/handlers/menu/export/tests/app_workspace.md) |
| called_by | [a_child_only_on_disk_is_stitched_without_being_opened](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_only_on_disk_is_stitched_without_being_opened.md) |
| called_by | [a_child_that_exists_but_will_not_parse_is_not_called_missing](/crates/oxide-app/src/app/handlers/menu/export/tests/a_child_that_exists_but_will_not_parse_is_not_called_missing.md) |
| called_by | [a_page_the_root_does_reach_is_not_reported_as_a_shortfall](/crates/oxide-app/src/app/handlers/menu/export/tests/a_page_the_root_does_reach_is_not_reported_as_a_shortfall.md) |
| called_by | [a_project_export_roots_at_the_project_root_not_the_active_child](/crates/oxide-app/src/app/handlers/menu/export/tests/a_project_export_roots_at_the_project_root_not_the_active_child.md) |
| called_by | [a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths](/crates/oxide-app/src/app/handlers/menu/export/tests/a_stale_persisted_dir_does_not_desync_ownership_from_the_sheet_paths.md) |
| called_by | [a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem](/crates/oxide-app/src/app/handlers/menu/export/tests/diagnosis/a_listed_page_with_no_file_is_not_diagnosed_as_a_graph_problem.md) |
| called_by | [grandchild_resolves_through_two_hops_to_the_project](/crates/oxide-app/src/app/handlers/menu/export/tests/grandchild_resolves_through_two_hops_to_the_project.md) |
| called_by | [hierarchical_child_sheet_exports_its_owning_project](/crates/oxide-app/src/app/handlers/menu/export/tests/hierarchical_child_sheet_exports_its_owning_project.md) |
| called_by | [listed_project_sheet_exports_the_whole_project](/crates/oxide-app/src/app/handlers/menu/export/tests/listed_project_sheet_exports_the_whole_project.md) |
| called_by | [loose_export_page_order_is_stable_across_rebuilds](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_export_page_order_is_stable_across_rebuilds.md) |
| called_by | [loose_schematic_exports_itself_not_the_sticky_projects_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/loose_schematic_exports_itself_not_the_sticky_projects_sheets.md) |
| called_by | [root_active_netlist_contains_a_child_absent_from_data_sheets](/crates/oxide-app/src/app/handlers/menu/export/tests/root_active_netlist_contains_a_child_absent_from_data_sheets.md) |
| called_by | [schematic_inside_the_project_directory_but_unlisted_stays_loose](/crates/oxide-app/src/app/handlers/menu/export/tests/schematic_inside_the_project_directory_but_unlisted_stays_loose.md) |
