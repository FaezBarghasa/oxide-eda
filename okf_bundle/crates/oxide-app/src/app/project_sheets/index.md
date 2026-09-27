# project_sheets

## Classs

- [AssembledGraph](AssembledGraph.md) — The result of [`project_graph`] re-keying [`ProjectSheetSet::sheets`]
- [ProjectSheetSet](ProjectSheetSet.md) — The sheets a project consists of, plus what could not be made sense of

## Functions

- [a_diamond_sharing_one_child_file_is_not_a_collision](a_diamond_sharing_one_child_file_is_not_a_collision.md) — [test]
- [absolute_child_reference_outside_the_root_is_rejected](absolute_child_reference_outside_the_root_is_rejected.md) — [test]
- [an_out_of_tree_namesake_cannot_evict_the_root](an_out_of_tree_namesake_cannot_evict_the_root.md) — #536 — the root must survive a `SheetKey` collision.
- [assemble_active_project_sheets](assemble_active_project_sheets.md) — The declared page paths of the project owning the active document, plus
- [assemble_project_sheets](assemble_project_sheets.md) — Every sheet this project consists of: the declared `pages` **union**
- [case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity](case_fold_collision_between_two_loaded_paths_is_the_remaining_ambiguity.md) — [test]
- [child_ref](child_ref.md)
- [empty](empty.md) — No project in view (no active document).
- [empty](empty_1.md) — No project in view (no active document).
- [empty_child_reference_resolves_to_none](empty_child_reference_resolves_to_none.md) — [test]
- [empty_parent_dir_rejects_an_escaping_reference](empty_parent_dir_rejects_an_escaping_reference.md) — [test]
- [grandchild_resolves_beside_its_parent_not_the_project_root](grandchild_resolves_beside_its_parent_not_the_project_root.md) — [test]
- [interior_dotdot_that_stays_inside_the_root_still_resolves](interior_dotdot_that_stays_inside_the_root_still_resolves.md) — [test]
- [key](key.md) — A [`oxide_net::SheetKey`] from a literal already in the normalized
- [lexically_normalize](lexically_normalize.md) — Lexically normalize `path` — resolve `.` and `..` components without
- [load_sheet](load_sheet.md) — Read one sheet: the live engine snapshot when the file is open as a tab (so
- [navigation_and_project_graph_agree_on_the_same_reference](navigation_and_project_graph_agree_on_the_same_reference.md) — [test]
- [ordered_project_sheet_paths](ordered_project_sheet_paths.md) — The sheet-walk order every whole-project Annotate operation must agree
- [pages_are_ordered_by_sheet_key_not_by_absolute_path](pages_are_ordered_by_sheet_key_not_by_absolute_path.md) — [test]
- [project_graph](project_graph.md) — Re-key `sheets` (the app's `path → SchematicSheet` set) into the
- [project_root_sheet_path](project_root_sheet_path.md) — Absolute path of a project's root schematic — its declared
- [project_roots](project_roots.md) — The entry points [`oxide_net::build_project_netlist`] must walk: the
- [resolve_child_reference](resolve_child_reference.md) — Resolve a `ChildSheet.filename` reference against the directory of the sheet
- [resolves_a_bare_child_reference_against_the_parent_dir](resolves_a_bare_child_reference_against_the_parent_dir.md) — [test]
- [same_basename_children_in_different_dirs_do_not_collide](same_basename_children_in_different_dirs_do_not_collide.md) — [test]
- [same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file](same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file.md) — [test]
- [sheet](sheet.md)
- [sheet_key](sheet_key.md) — The [`oxide_net::SheetKey`] for `path` — its path relative to `base`,
- [sheet_key_is_relative_to_the_base_dir](sheet_key_is_relative_to_the_base_dir.md) — [test]
- [sibling_child_reference_still_resolves](sibling_child_reference_still_resolves.md) — [test]
- [stitch_issue_message](stitch_issue_message.md) — A one-line, user-facing message for a cross-sheet stitch issue (ADR-0002 D7,
- [the_namesake_the_root_displaces_is_still_reported](the_namesake_the_root_displaces_is_still_reported.md) — The exemption does not hide the collision — the outsider is still
- [traversal_child_reference_outside_the_root_is_rejected](traversal_child_reference_outside_the_root_is_rejected.md) — [test]
- [unreferenced_and_unloadable_sheets_are_absent_from_resolution](unreferenced_and_unloadable_sheets_are_absent_from_resolution.md) — [test]
- [walk](walk.md) — Breadth-first load of `seeds` and their `child_sheets` descendants into
- [without_a_root_the_winner_is_still_sorted_first_wins](without_a_root_the_winner_is_still_sorted_first_wins.md) — `None` is what ERC passes, and it must keep the old rule exactly:
