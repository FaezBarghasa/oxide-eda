---
okf_version: "0.2"
type: Function
title: ingest_sheet
description: "Replace all entries for `sheet` under `project` with `refs`."
resource: crates/oxide-library/src/where_used.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/where_used/ingest_sheet
language: rust
---

# ingest_sheet

Replace all entries for `sheet` under `project` with `refs`.

## Signature

```rust
impl WhereUsedIndex { pub fn ingest_sheet(&mut self, project: &Path, sheet: &Path, refs: &[(RowId, String)]) }
```

## Visibility

- `pub`

## Docstring

Replace all entries for `sheet` under `project` with `refs`.

Called when a sheet is opened, saved, or otherwise re-scanned by the
consumer. Earlier entries for the same `(project, sheet)` are dropped
before the new refs are inserted, so the index never accumulates stale
duplicates from re-ingestion.

## Source
Lines 93–110 in `crates/oxide-library/src/where_used.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [where_used](/crates/oxide-library/src/where_used.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
