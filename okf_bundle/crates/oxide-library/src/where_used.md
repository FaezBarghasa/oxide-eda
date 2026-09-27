---
okf_version: "0.2"
type: Module
title: where_used
description: "Where-used reverse index — keyed by `RowId` for the DBLib model."
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used
language: rust
---

# where_used

Where-used reverse index — keyed by `RowId` for the DBLib model.

## Docstring

Where-used reverse index — keyed by `RowId` for the DBLib model.

Pure data structure. The consumer (oxide-app) pushes references in via
`ingest_sheet` whenever a sheet is opened or saved, and drops a project's
entries via `drop_project` when the project closes. There is no filesystem
walking here — that is the consumer's job.

Lookups by `RowId` return the list of every (project, sheet, instance)
site where the row is used. UI consumes the results.

Storage shape: a `HashMap<PathBuf /* project */, HashMap<PathBuf /* sheet */, Vec<Entry>>>`.
Re-ingesting a sheet replaces (does not append) its previous entries — this
is what makes the index incremental and idempotent under repeated open/save.

Per `v0.9-refactor-2-plan.md` §6 step 1.8, the index is keyed by
[`RowId`] (component-table row), not by the legacy `ComponentId`. The
`primitive_to_rows` reverse index is rebuilt by adapters via
`iter_rows()` — see [`Self::rebuild_from_rows`].

## Relationships

| Type | Target |
|------|--------|
| related | [UseSite](/crates/oxide-library/src/where_used/UseSite.md) |
| related | [Entry](/crates/oxide-library/src/where_used/Entry.md) |
| related | [WhereUsedIndex](/crates/oxide-library/src/where_used/WhereUsedIndex.md) |
| related | [new](/crates/oxide-library/src/where_used/new.md) |
| related | [ingest_sheet](/crates/oxide-library/src/where_used/ingest_sheet.md) |
| related | [drop_project](/crates/oxide-library/src/where_used/drop_project.md) |
| related | [rebuild_from_rows](/crates/oxide-library/src/where_used/rebuild_from_rows.md) |
| related | [ingest_row](/crates/oxide-library/src/where_used/ingest_row.md) |
| related | [add_primitive_links](/crates/oxide-library/src/where_used/add_primitive_links.md) |
| related | [rows_for_primitive](/crates/oxide-library/src/where_used/rows_for_primitive.md) |
| related | [where_used](/crates/oxide-library/src/where_used/where_used.md) |
| related | [new](/crates/oxide-library/src/where_used/new.md) |
| related | [ingest_sheet](/crates/oxide-library/src/where_used/ingest_sheet.md) |
| related | [drop_project](/crates/oxide-library/src/where_used/drop_project.md) |
| related | [rebuild_from_rows](/crates/oxide-library/src/where_used/rebuild_from_rows.md) |
| related | [ingest_row](/crates/oxide-library/src/where_used/ingest_row.md) |
| related | [add_primitive_links](/crates/oxide-library/src/where_used/add_primitive_links.md) |
| related | [rows_for_primitive](/crates/oxide-library/src/where_used/rows_for_primitive.md) |
| related | [where_used](/crates/oxide-library/src/where_used/where_used.md) |
| related | [_assert_send_not_sync](/crates/oxide-library/src/where_used/assert_send_not_sync.md) |
| related | [is_send](/crates/oxide-library/src/where_used/is_send.md) |
| related | [fixture_row](/crates/oxide-library/src/where_used/fixture_row.md) |
| related | [new_index_is_empty](/crates/oxide-library/src/where_used/new_index_is_empty.md) |
| related | [ingesting_empty_refs_clears_a_previous_sheet_entry](/crates/oxide-library/src/where_used/ingesting_empty_refs_clears_a_previous_sheet_entry.md) |
| related | [rows_for_primitive_returns_referencing_row](/crates/oxide-library/src/where_used/rows_for_primitive_returns_referencing_row.md) |
| related | [rebuild_from_rows_replaces_state](/crates/oxide-library/src/where_used/rebuild_from_rows_replaces_state.md) |
| related | [ingest_row_replaces_prior_primitive_links](/crates/oxide-library/src/where_used/ingest_row_replaces_prior_primitive_links.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
