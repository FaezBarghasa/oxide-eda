---
okf_version: "0.2"
type: Module
title: 0005_tabular_components
description: v0.9-refactor-2 (WS-3 / WS-4) — DBLib row model.
resource: crates/oxide-library-server/migrations/0005_tabular_components.sql
tags:
  - "lang:sql"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0005_tabular_components
language: sql
---

# 0005_tabular_components

v0.9-refactor-2 (WS-3 / WS-4) — DBLib row model.

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

## Relationships

| Type | Target |
|------|--------|
| related | [component_rows](/crates/oxide-library-server/migrations/0005_tabular_components/component_rows.md) |
| related | [idx_component_rows_pn](/crates/oxide-library-server/migrations/0005_tabular_components/idx_component_rows_pn.md) |
