---
okf_version: "0.2"
type: Module
title: project_sheets
description: "Shared assembly of the project's sheet view — [`assemble_project_sheets`]"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets
language: rust
---

# project_sheets

Shared assembly of the project's sheet view — [`assemble_project_sheets`]

## Docstring

Shared assembly of the project's sheet view — [`assemble_project_sheets`]
answers "what sheets does this project consist of", and [`project_graph`]
re-keys that answer by resolved path the way
[`oxide_net::build_project_netlist`] and ERC read it (ADR-0002 D8, #466).

There is exactly one assembler on purpose. Five operations ask this
question — the export scope, the cached canvas/ERC netlist, the ERC run,
annotate and the duplicate-designator reset — and each used to build its own
input set from slightly different rules. They disagreed: on the same state
at the same instant, one reported a `MissingChild` for a sheet another had
already stitched in (#406).

## Relationships

| Type | Target |
|------|--------|
| related | [ProjectSheetSet](/crates/oxide-app/src/app/project_sheets/ProjectSheetSet.md) |
| related | [empty](/crates/oxide-app/src/app/project_sheets/empty.md) |
| related | [empty](/crates/oxide-app/src/app/project_sheets/empty.md) |
| related | [assemble_active_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_active_project_sheets.md) |
| related | [ordered_project_sheet_paths](/crates/oxide-app/src/app/project_sheets/ordered_project_sheet_paths.md) |
| related | [project_root_sheet_path](/crates/oxide-app/src/app/project_sheets/project_root_sheet_path.md) |
| related | [load_sheet](/crates/oxide-app/src/app/project_sheets/load_sheet.md) |
| related | [assemble_project_sheets](/crates/oxide-app/src/app/project_sheets/assemble_project_sheets.md) |
| related | [walk](/crates/oxide-app/src/app/project_sheets/walk.md) |
| related | [AssembledGraph](/crates/oxide-app/src/app/project_sheets/AssembledGraph.md) |
| related | [project_roots](/crates/oxide-app/src/app/project_sheets/project_roots.md) |
| related | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| related | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
| related | [resolve_child_reference](/crates/oxide-app/src/app/project_sheets/resolve_child_reference.md) |
| related | [lexically_normalize](/crates/oxide-app/src/app/project_sheets/lexically_normalize.md) |
| related | [stitch_issue_message](/crates/oxide-app/src/app/project_sheets/stitch_issue_message.md) |
| related | [child_ref](/crates/oxide-app/src/app/project_sheets/child_ref.md) |
| related | [key](/crates/oxide-app/src/app/project_sheets/key.md) |
| related | [sheet](/crates/oxide-app/src/app/project_sheets/sheet.md) |
| related | [same_basename_children_in_different_dirs_do_not_collide](/crates/oxide-app/src/app/project_sheets/same_basename_children_in_different_dirs_do_not_collide.md) |
| related | [same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file](/crates/oxide-app/src/app/project_sheets/same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file.md) |
| related | [a_diamond_sharing_one_child_file_is_not_a_collision](/crates/oxide-app/src/app/project_sheets/a_diamond_sharing_one_child_file_is_not_a_collision.md) |
| related | [case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity](/crates/oxide-app/src/app/project_sheets/case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity.md) |
| related | [resolves_a_bare_child_reference_against_the_parent_dir](/crates/oxide-app/src/app/project_sheets/resolves_a_bare_child_reference_against_the_parent_dir.md) |
| related | [unreferenced_and_unloadable_sheets_are_absent_from_resolution](/crates/oxide-app/src/app/project_sheets/unreferenced_and_unloadable_sheets_are_absent_from_resolution.md) |
| related | [sheet_key_is_relative_to_the_base_dir](/crates/oxide-app/src/app/project_sheets/sheet_key_is_relative_to_the_base_dir.md) |
| related | [grandchild_resolves_beside_its_parent_not_the_project_root](/crates/oxide-app/src/app/project_sheets/grandchild_resolves_beside_its_parent_not_the_project_root.md) |
| related | [absolute_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/project_sheets/absolute_child_reference_outside_the_root_is_rejected.md) |
| related | [traversal_child_reference_outside_the_root_is_rejected](/crates/oxide-app/src/app/project_sheets/traversal_child_reference_outside_the_root_is_rejected.md) |
| related | [sibling_child_reference_still_resolves](/crates/oxide-app/src/app/project_sheets/sibling_child_reference_still_resolves.md) |
| related | [empty_child_reference_resolves_to_none](/crates/oxide-app/src/app/project_sheets/empty_child_reference_resolves_to_none.md) |
| related | [empty_parent_dir_rejects_an_escaping_reference](/crates/oxide-app/src/app/project_sheets/empty_parent_dir_rejects_an_escaping_reference.md) |
| related | [interior_dotdot_that_stays_inside_the_root_still_resolves](/crates/oxide-app/src/app/project_sheets/interior_dotdot_that_stays_inside_the_root_still_resolves.md) |
| related | [navigation_and_project_graph_agree_on_the_same_reference](/crates/oxide-app/src/app/project_sheets/navigation_and_project_graph_agree_on_the_same_reference.md) |
| related | [pages_are_ordered_by_sheet_key_not_by_absolute_path](/crates/oxide-app/src/app/project_sheets/pages_are_ordered_by_sheet_key_not_by_absolute_path.md) |
| related | [an_out_of_tree_namesake_cannot_evict_the_root](/crates/oxide-app/src/app/project_sheets/an_out_of_tree_namesake_cannot_evict_the_root.md) |
| related | [the_namesake_the_root_displaces_is_still_reported](/crates/oxide-app/src/app/project_sheets/the_namesake_the_root_displaces_is_still_reported.md) |
| related | [without_a_root_the_winner_is_still_sorted_first_wins](/crates/oxide-app/src/app/project_sheets/without_a_root_the_winner_is_still_sorted_first_wins.md) |
