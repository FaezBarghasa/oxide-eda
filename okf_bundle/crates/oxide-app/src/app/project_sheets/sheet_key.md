---
okf_version: "0.2"
type: Function
title: sheet_key
description: "The [`oxide_net::SheetKey`] for `path` — its path relative to `base`,"
resource: crates/oxide-app/src/app/project_sheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/project_sheets/sheet_key
language: rust
---

# sheet_key

The [`oxide_net::SheetKey`] for `path` — its path relative to `base`,

## Signature

```rust
pub(crate) fn sheet_key(path: &Path, base: Option<&Path>) -> oxide_net::SheetKey
```

## Visibility

- `pub(crate)`

## Docstring

The [`oxide_net::SheetKey`] for `path` — its path relative to `base`,
normalized by [`path_key`], falling back to the bare basename when `path`
does not live under `base` (or there is no `base` — a loose document with
no project). Keying a project by resolved path rather than by the bare
reference string is the correctness #466 is after: two parents in
different directories may name a child by the identical string and mean
two different files.

Generalizes the old `root_reference_name`, which derived a key for the
root sheet only, to every sheet.

Normalization is exactly [`path_key`]'s and no more: separators to `/` and
case folded **on Windows only** (`cfg!(windows)`). That is a compile-time
host-family proxy, not a per-volume answer, so it under-folds on a
case-insensitive macOS APFS volume — `A.snxsch` and `a.snxsch` there are
one file the app keys as two — and over-folds on a Windows directory with
NTFS per-directory case sensitivity enabled. Both are pre-existing
`path_key` behaviour that this function inherits rather than introduces;
tightening it belongs with `path_key`, which project membership also
depends on.

## Source
Lines 343–350 in `crates/oxide-app/src/app/project_sheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [project_sheets](/crates/oxide-app/src/app/project_sheets.md) |
| calls | [path_key](/crates/oxide-app/src/app/state/scope/path_key.md) |
| called_by | [handle_run_erc](/crates/oxide-app/src/app/handlers/erc/erc_run/handle_run_erc.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
| called_by | [refresh_project_netlist](/crates/oxide-app/src/app/mutation_gateway/refresh_project_netlist.md) |
| called_by | [a_diamond_sharing_one_child_file_is_not_a_collision](/crates/oxide-app/src/app/project_sheets/a_diamond_sharing_one_child_file_is_not_a_collision.md) |
| called_by | [navigation_and_project_graph_agree_on_the_same_reference](/crates/oxide-app/src/app/project_sheets/navigation_and_project_graph_agree_on_the_same_reference.md) |
| called_by | [pages_are_ordered_by_sheet_key_not_by_absolute_path](/crates/oxide-app/src/app/project_sheets/pages_are_ordered_by_sheet_key_not_by_absolute_path.md) |
| called_by | [project_graph](/crates/oxide-app/src/app/project_sheets/project_graph.md) |
| called_by | [project_roots](/crates/oxide-app/src/app/project_sheets/project_roots.md) |
| called_by | [resolves_a_bare_child_reference_against_the_parent_dir](/crates/oxide-app/src/app/project_sheets/resolves_a_bare_child_reference_against_the_parent_dir.md) |
| called_by | [same_basename_children_in_different_dirs_do_not_collide](/crates/oxide-app/src/app/project_sheets/same_basename_children_in_different_dirs_do_not_collide.md) |
| called_by | [same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file](/crates/oxide-app/src/app/project_sheets/same_reference_string_across_different_parent_dirs_stitches_each_from_its_own_file.md) |
| called_by | [unreferenced_and_unloadable_sheets_are_absent_from_resolution](/crates/oxide-app/src/app/project_sheets/unreferenced_and_unloadable_sheets_are_absent_from_resolution.md) |
