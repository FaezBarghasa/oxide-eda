---
okf_version: "0.2"
type: Function
title: project_graph
description: "Re-key `sheets` (the app's `path → SchematicSheet` set) into the"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/project_graph
language: rust
---

# project_graph

Re-key `sheets` (the app's `path → SchematicSheet` set) into the

## Signature

```rust
pub(crate) fn project_graph(
    sheets: &HashMap<PathBuf, SchematicSheet>,
    base_dir: Option<&Path>,
    root: Option<&Path>,
) -> AssembledGraph
```

## Visibility

- `pub(crate)`

## Docstring

Re-key `sheets` (the app's `path → SchematicSheet` set) into the
[`oxide_net::ProjectGraph`] shape — every sheet under its own
[`oxide_net::SheetKey`], plus a per-parent `ChildSheet.filename ->
SheetKey` resolution map — so a cross-directory same-filename child
stitches from its own file instead of colliding with another parent's
(#466).

`base_dir` is the fixed base every key is made relative to: the project
directory when the sheet set belongs to one, else the root sheet's own
directory (a loose document still keys its children relative to itself).

Two distinct loaded paths can still collapse onto the same `SheetKey` —
[`sheet_key`] normalizes, and on a case-insensitive host it also folds
case, so `A.snxsch` and `a.snxsch` assemble to one key. Sorted-path
first-wins keeps which one survives deterministic; the loser is reported as
[`oxide_net::StitchIssue::SheetKeyCollision`] and contributes nothing to
the netlist, which is silent without that issue.

`root` is the path the caller is about to stitch from, and it is exempt
from losing such a collision (#536). Sorted-path first-wins had no notion
of a root, so `/proj/Top.snxsch` beat `/proj/top.snxsch` on a plain byte
compare (`T` = `0x54` sorts before `t` = `0x74`) and evicted the root — at
which point both netlist callers found no root key and bailed, producing
no `.net`, no connectivity for `NET_NAME()`, and no explanation. A root
that loses to a stray case-variant copy is never what the user meant, so
the copy is dropped instead; it is still reported as a collision, and the
project still exports. Pass `None` when the assembly has no single entry
point (ERC reports on every sheet regardless of reachability).

## Source
Lines 380–441 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [sheet_key](/crates/oxide-app/src/app/project_sheets/sheet_key.md) |
| calls | [resolve_child_reference](/crates/oxide-app/src/app/project_sheets/resolve_child_reference.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |
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
