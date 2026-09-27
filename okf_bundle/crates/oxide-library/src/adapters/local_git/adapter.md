---
okf_version: "0.2"
type: Module
title: adapter
description: "`LibraryAdapter` trait implementation for `LocalGitAdapter`."
resource: crates/oxide-library/src/adapters/local_git/adapter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/local_git/adapter
language: rust
---

# adapter

`LibraryAdapter` trait implementation for `LocalGitAdapter`.

## Docstring

`LibraryAdapter` trait implementation for `LocalGitAdapter`.

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-library/src/adapters/local_git/adapter/manifest.md) |
| related | [library_file](/crates/oxide-library/src/adapters/local_git/adapter/library_file.md) |
| related | [root_dir](/crates/oxide-library/src/adapters/local_git/adapter/root_dir.md) |
| related | [library_file_path](/crates/oxide-library/src/adapters/local_git/adapter/library_file_path.md) |
| related | [list_tables](/crates/oxide-library/src/adapters/local_git/adapter/list_tables.md) |
| related | [rename_table](/crates/oxide-library/src/adapters/local_git/adapter/rename_table.md) |
| related | [delete_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/delete_empty_table.md) |
| related | [library_classes](/crates/oxide-library/src/adapters/local_git/adapter/library_classes.md) |
| related | [update_library_classes](/crates/oxide-library/src/adapters/local_git/adapter/update_library_classes.md) |
| related | [add_library_class](/crates/oxide-library/src/adapters/local_git/adapter/add_library_class.md) |
| related | [remove_library_class](/crates/oxide-library/src/adapters/local_git/adapter/remove_library_class.md) |
| related | [rename_library_class](/crates/oxide-library/src/adapters/local_git/adapter/rename_library_class.md) |
| related | [create_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/create_empty_table.md) |
| related | [read_table](/crates/oxide-library/src/adapters/local_git/adapter/read_table.md) |
| related | [iter_rows](/crates/oxide-library/src/adapters/local_git/adapter/iter_rows.md) |
| related | [read_row](/crates/oxide-library/src/adapters/local_git/adapter/read_row.md) |
| related | [read_row_by_pn](/crates/oxide-library/src/adapters/local_git/adapter/read_row_by_pn.md) |
| related | [insert_row](/crates/oxide-library/src/adapters/local_git/adapter/insert_row.md) |
| related | [update_row](/crates/oxide-library/src/adapters/local_git/adapter/update_row.md) |
| related | [delete_row](/crates/oxide-library/src/adapters/local_git/adapter/delete_row.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/local_git/adapter/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/local_git/adapter/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/local_git/adapter/get_sim.md) |
| related | [save_symbol](/crates/oxide-library/src/adapters/local_git/adapter/save_symbol.md) |
| related | [save_footprint](/crates/oxide-library/src/adapters/local_git/adapter/save_footprint.md) |
| related | [save_sim](/crates/oxide-library/src/adapters/local_git/adapter/save_sim.md) |
| related | [list_symbols](/crates/oxide-library/src/adapters/local_git/adapter/list_symbols.md) |
| related | [list_footprints](/crates/oxide-library/src/adapters/local_git/adapter/list_footprints.md) |
| related | [list_sims](/crates/oxide-library/src/adapters/local_git/adapter/list_sims.md) |
| related | [root_path](/crates/oxide-library/src/adapters/local_git/adapter/root_path.md) |
| related | [commit_external_change](/crates/oxide-library/src/adapters/local_git/adapter/commit_external_change.md) |
| related | [history](/crates/oxide-library/src/adapters/local_git/adapter/history.md) |
| related | [manifest](/crates/oxide-library/src/adapters/local_git/adapter/manifest.md) |
| related | [library_file](/crates/oxide-library/src/adapters/local_git/adapter/library_file.md) |
| related | [root_dir](/crates/oxide-library/src/adapters/local_git/adapter/root_dir.md) |
| related | [library_file_path](/crates/oxide-library/src/adapters/local_git/adapter/library_file_path.md) |
| related | [list_tables](/crates/oxide-library/src/adapters/local_git/adapter/list_tables.md) |
| related | [rename_table](/crates/oxide-library/src/adapters/local_git/adapter/rename_table.md) |
| related | [delete_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/delete_empty_table.md) |
| related | [library_classes](/crates/oxide-library/src/adapters/local_git/adapter/library_classes.md) |
| related | [update_library_classes](/crates/oxide-library/src/adapters/local_git/adapter/update_library_classes.md) |
| related | [add_library_class](/crates/oxide-library/src/adapters/local_git/adapter/add_library_class.md) |
| related | [remove_library_class](/crates/oxide-library/src/adapters/local_git/adapter/remove_library_class.md) |
| related | [rename_library_class](/crates/oxide-library/src/adapters/local_git/adapter/rename_library_class.md) |
| related | [create_empty_table](/crates/oxide-library/src/adapters/local_git/adapter/create_empty_table.md) |
| related | [read_table](/crates/oxide-library/src/adapters/local_git/adapter/read_table.md) |
| related | [iter_rows](/crates/oxide-library/src/adapters/local_git/adapter/iter_rows.md) |
| related | [read_row](/crates/oxide-library/src/adapters/local_git/adapter/read_row.md) |
| related | [read_row_by_pn](/crates/oxide-library/src/adapters/local_git/adapter/read_row_by_pn.md) |
| related | [insert_row](/crates/oxide-library/src/adapters/local_git/adapter/insert_row.md) |
| related | [update_row](/crates/oxide-library/src/adapters/local_git/adapter/update_row.md) |
| related | [delete_row](/crates/oxide-library/src/adapters/local_git/adapter/delete_row.md) |
| related | [get_symbol](/crates/oxide-library/src/adapters/local_git/adapter/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapters/local_git/adapter/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapters/local_git/adapter/get_sim.md) |
| related | [save_symbol](/crates/oxide-library/src/adapters/local_git/adapter/save_symbol.md) |
| related | [save_footprint](/crates/oxide-library/src/adapters/local_git/adapter/save_footprint.md) |
| related | [save_sim](/crates/oxide-library/src/adapters/local_git/adapter/save_sim.md) |
| related | [list_symbols](/crates/oxide-library/src/adapters/local_git/adapter/list_symbols.md) |
| related | [list_footprints](/crates/oxide-library/src/adapters/local_git/adapter/list_footprints.md) |
| related | [list_sims](/crates/oxide-library/src/adapters/local_git/adapter/list_sims.md) |
| related | [root_path](/crates/oxide-library/src/adapters/local_git/adapter/root_path.md) |
| related | [commit_external_change](/crates/oxide-library/src/adapters/local_git/adapter/commit_external_change.md) |
| related | [history](/crates/oxide-library/src/adapters/local_git/adapter/history.md) |
