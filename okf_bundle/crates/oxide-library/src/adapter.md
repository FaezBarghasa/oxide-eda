---
okf_version: "0.2"
type: Module
title: adapter
description: "`LibraryAdapter` — the trait every storage flavour implements."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter
language: rust
---

# adapter

`LibraryAdapter` — the trait every storage flavour implements.

## Docstring

`LibraryAdapter` — the trait every storage flavour implements.

Per `v0.9-refactor-2-plan.md` §7, the trait is row-shaped: the legacy
`get_component` / `get_revision` / `save_revision` methods are gone,
replaced by table CRUD that targets [`ComponentRow`] (the DBLib model).
Every adapter — LocalGit (TSV) or Database (JSONB) — answers the same
row-oriented surface.

## Relationships

| Type | Target |
|------|--------|
| related | [LibraryError](/crates/oxide-library/src/adapter/LibraryError.md) |
| related | [FieldSet](/crates/oxide-library/src/adapter/FieldSet.md) |
| related | [LibraryQuery](/crates/oxide-library/src/adapter/LibraryQuery.md) |
| related | [ComponentSummary](/crates/oxide-library/src/adapter/ComponentSummary.md) |
| related | [PrimitiveSummary](/crates/oxide-library/src/adapter/PrimitiveSummary.md) |
| related | [HistoryEntry](/crates/oxide-library/src/adapter/HistoryEntry.md) |
| related | [LibraryAdapter](/crates/oxide-library/src/adapter/LibraryAdapter.md) |
| related | [library_id](/crates/oxide-library/src/adapter/library_id.md) |
| related | [library_file](/crates/oxide-library/src/adapter/library_file.md) |
| related | [root_dir](/crates/oxide-library/src/adapter/root_dir.md) |
| related | [library_file_path](/crates/oxide-library/src/adapter/library_file_path.md) |
| related | [list_tables](/crates/oxide-library/src/adapter/list_tables.md) |
| related | [create_empty_table](/crates/oxide-library/src/adapter/create_empty_table.md) |
| related | [delete_empty_table](/crates/oxide-library/src/adapter/delete_empty_table.md) |
| related | [rename_table](/crates/oxide-library/src/adapter/rename_table.md) |
| related | [library_classes](/crates/oxide-library/src/adapter/library_classes.md) |
| related | [update_library_classes](/crates/oxide-library/src/adapter/update_library_classes.md) |
| related | [add_library_class](/crates/oxide-library/src/adapter/add_library_class.md) |
| related | [remove_library_class](/crates/oxide-library/src/adapter/remove_library_class.md) |
| related | [rename_library_class](/crates/oxide-library/src/adapter/rename_library_class.md) |
| related | [read_table](/crates/oxide-library/src/adapter/read_table.md) |
| related | [iter_rows](/crates/oxide-library/src/adapter/iter_rows.md) |
| related | [read_row](/crates/oxide-library/src/adapter/read_row.md) |
| related | [read_row_by_pn](/crates/oxide-library/src/adapter/read_row_by_pn.md) |
| related | [insert_row](/crates/oxide-library/src/adapter/insert_row.md) |
| related | [update_row](/crates/oxide-library/src/adapter/update_row.md) |
| related | [delete_row](/crates/oxide-library/src/adapter/delete_row.md) |
| related | [try_lock](/crates/oxide-library/src/adapter/try_lock.md) |
| related | [release_lock](/crates/oxide-library/src/adapter/release_lock.md) |
| related | [get_symbol](/crates/oxide-library/src/adapter/get_symbol.md) |
| related | [get_footprint](/crates/oxide-library/src/adapter/get_footprint.md) |
| related | [get_sim](/crates/oxide-library/src/adapter/get_sim.md) |
| related | [save_symbol](/crates/oxide-library/src/adapter/save_symbol.md) |
| related | [save_footprint](/crates/oxide-library/src/adapter/save_footprint.md) |
| related | [save_sim](/crates/oxide-library/src/adapter/save_sim.md) |
| related | [list_symbols](/crates/oxide-library/src/adapter/list_symbols.md) |
| related | [list_footprints](/crates/oxide-library/src/adapter/list_footprints.md) |
| related | [list_sims](/crates/oxide-library/src/adapter/list_sims.md) |
| related | [root_path](/crates/oxide-library/src/adapter/root_path.md) |
| related | [commit_external_change](/crates/oxide-library/src/adapter/commit_external_change.md) |
| related | [history](/crates/oxide-library/src/adapter/history.md) |
| related | [library_adapter_is_object_safe](/crates/oxide-library/src/adapter/library_adapter_is_object_safe.md) |
| related | [_accepts_dyn](/crates/oxide-library/src/adapter/accepts_dyn.md) |
| related | [library_query_default_is_empty](/crates/oxide-library/src/adapter/library_query_default_is_empty.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
