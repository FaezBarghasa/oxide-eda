---
okf_version: "0.2"
type: Function
title: sheet
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/sheet
language: rust
---

# sheet

## Signature

```rust
fn sheet(uuid: u128, children: &[&str]) -> SchematicSheet
```

## Source
Lines 617–639 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [child_ref](/crates/oxide-app/src/app/project_sheets/child_ref.md) |
| called_by | [a_diamond_sharing_one_child_file_is_not_a_collision](/crates/oxide-app/src/app/project_sheets/a_diamond_sharing_one_child_file_is_not_a_collision.md) |
| called_by | [an_out_of_tree_namesake_cannot_evict_the_root](/crates/oxide-app/src/app/project_sheets/an_out_of_tree_namesake_cannot_evict_the_root.md) |
| called_by | [case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity](/crates/oxide-app/src/app/project_sheets/case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity.md) |
| called_by | [navigation_and_project_graph_agree_on_the_same_reference](/crates/oxide-app/src/app/project_sheets/navigation_and_project_graph_agree_on_the_same_reference.md) |
| called_by | [pages_are_ordered_by_sheet_key_not_by_absolute_path](/crates/oxide-app/src/app/project_sheets/pages_are_ordered_by_sheet_key_not_by_absolute_path.md) |
| called_by | [resolves_a_bare_child_reference_against_the_parent_dir](/crates/oxide-app/src/app/project_sheets/resolves_a_bare_child_reference_against_the_parent_dir.md) |
| called_by | [same_basename_children_in_different_dirs_do_not_collide](/crates/oxide-app/src/app/project_sheets/same_basename_children_in_different_dirs_do_not_collide.md) |
| called_by | [same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file](/crates/oxide-app/src/app/project_sheets/same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file.md) |
| called_by | [the_namesake_the_root_displaces_is_still_reported](/crates/oxide-app/src/app/project_sheets/the_namesake_the_root_displaces_is_still_reported.md) |
| called_by | [unreferenced_and_unloadable_sheets_are_absent_from_resolution](/crates/oxide-app/src/app/project_sheets/unreferenced_and_unloadable_sheets_are_absent_from_resolution.md) |
| called_by | [without_a_root_the_winner_is_still_sorted_first_wins](/crates/oxide-app/src/app/project_sheets/without_a_root_the_winner_is_still_sorted_first_wins.md) |
