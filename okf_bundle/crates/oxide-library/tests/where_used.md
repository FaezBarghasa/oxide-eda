---
okf_version: "0.2"
type: Module
title: where_used
description: Integration tests for the where-used reverse index.
resource: crates/oxide-library/tests/where_used.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/where_used
language: rust
---

# where_used

Integration tests for the where-used reverse index.

## Docstring

Integration tests for the where-used reverse index.

The index is keyed by [`RowId`] (component-table row) under the
DBLib model — not by a `(uuid, Version)` pair. Verifies the
public contract:
- `ingest_sheet` registers row references for a (project, sheet) pair.
- `where_used(row_id)` returns every site.
- `drop_project(p)` removes every site under that project root.
- Re-ingesting a sheet replaces (not appends) its previous entries.

## Relationships

| Type | Target |
|------|--------|
| related | [where_used_returns_all_sites_for_a_row_across_sheets](/crates/oxide-library/tests/where_used/where_used_returns_all_sites_for_a_row_across_sheets.md) |
| related | [drop_project_removes_all_sites_under_that_project](/crates/oxide-library/tests/where_used/drop_project_removes_all_sites_under_that_project.md) |
| related | [re_ingesting_a_sheet_replaces_its_previous_entries](/crates/oxide-library/tests/where_used/re_ingesting_a_sheet_replaces_its_previous_entries.md) |
