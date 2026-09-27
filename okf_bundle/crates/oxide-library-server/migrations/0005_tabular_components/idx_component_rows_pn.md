---
okf_version: "0.2"
type: Index
title: idx_component_rows_pn
description: The picker UI sorts rows alphabetically by internal_pn within a table; a
resource: crates/oxide-library-server/migrations/0005_tabular_components.sql
tags:
  - "lang:sql"
  - "type:Index"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0005_tabular_components/idx_component_rows_pn
language: sql
---

# idx_component_rows_pn

The picker UI sorts rows alphabetically by internal_pn within a table; a

## Signature

```sql
CREATE INDEX IF NOT EXISTS idx_component_rows_pn
    ON component_rows(library_id, table_name, internal_pn)
```

## Docstring

The picker UI sorts rows alphabetically by internal_pn within a table; a
composite index over (library_id, table_name, internal_pn) covers that
access pattern without forcing a full scan + in-memory sort.

## Source
Lines 35–36 in `crates/oxide-library-server/migrations/0005_tabular_components.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0005_tabular_components](/crates/oxide-library-server/migrations/0005_tabular_components.md) |
