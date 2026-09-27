---
okf_version: "0.2"
type: Table
title: component_rows
description: v0.9-refactor-2 (WS-3 / WS-4) — DBLib row model.
resource: crates/oxide-library-server/migrations/0005_tabular_components.sql
tags:
  - "lang:sql"
  - "type:Table"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0005_tabular_components/component_rows
language: sql
---

# component_rows

v0.9-refactor-2 (WS-3 / WS-4) — DBLib row model.

## Signature

```sql
CREATE TABLE IF NOT EXISTS component_rows (
    library_id     TEXT NOT NULL,
    table_name     TEXT NOT NULL,
    row_id         TEXT NOT NULL,
    internal_pn    TEXT NOT NULL,
    payload        T ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `library_id` | `TEXT` | `NOT NULL` |
| `table_name` | `TEXT` | `NOT NULL` |
| `row_id` | `TEXT` | `NOT NULL` |
| `internal_pn` | `TEXT` | `NOT NULL` |
| `payload` | `TEXT` | `NOT NULL` |
| `created_at` | `TEXT` | `NOT NULL` |
| `updated_at` | `TEXT` | `NOT NULL` |

## Docstring

v0.9-refactor-2 (WS-3 / WS-4) — DBLib row model.

Per `v0.9-refactor-2-plan.md` §2.1, components are now rows inside category
tables (Altium DBLib parity). The legacy per-revision schema in
`0001_initial.sql` and `004_primitives.sql` (components / revisions /
parameters / suppliers) is superseded by a single `component_rows` table
whose payload is the JSON-serialised `ComponentRow` struct from
`oxide-library`.

Layout intentionally mirrors the WS-D primitives tables: `(library_id, …)`
is the partition key, the row's `row_id` (UUIDv7 stringified) is the
intra-table identifier, and `payload` carries the canonical JSON. The
`table_name` column groups rows by category (e.g. "resistors",
"Discrete_Passives") matching the LocalGit `tables/<name>.tsv` filename
stem from `Manifest::table_for_class`.

Schema portability: same DDL applies to SQLite and Postgres — TEXT for
UUIDs, JSON, and timestamps. Postgres-native `uuid`/`jsonb` are deferred
until the cross-backend test matrix is in CI.

## Source
Lines 21–30 in `crates/oxide-library-server/migrations/0005_tabular_components.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0005_tabular_components](/crates/oxide-library-server/migrations/0005_tabular_components.md) |
