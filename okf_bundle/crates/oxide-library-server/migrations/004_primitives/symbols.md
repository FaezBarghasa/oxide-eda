---
okf_version: "0.2"
type: Table
title: symbols
description: v0.9 library refactor (WS-C step C3) — primitive storage tables.
resource: crates/oxide-library-server/migrations/004_primitives.sql
tags:
  - "lang:sql"
  - "type:Table"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library-server/migrations/004_primitives/symbols
language: sql
---

# symbols

v0.9 library refactor (WS-C step C3) — primitive storage tables.

## Signature

```sql
CREATE TABLE IF NOT EXISTS symbols (
    library_id     TEXT NOT NULL,
    uuid           TEXT NOT NULL,
    name           TEXT NOT NULL,
    payload        TEXT NOT NULL,           -- full Symbol JS ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `library_id` | `TEXT` | `NOT NULL` |
| `uuid` | `TEXT` | `NOT NULL` |
| `name` | `TEXT` | `NOT NULL` |
| `payload` | `TEXT` | `NOT NULL` |
| `created_at` | `TEXT` | `NOT NULL` |
| `updated_at` | `TEXT` | `NOT NULL` |

## Docstring

v0.9 library refactor (WS-C step C3) — primitive storage tables.

Reusable shape primitives (Symbol/Footprint/SimModel) addressed by a
(library_id, uuid) tuple per `v0.9-refactor-2-plan.md` §2 / §8.

Mirrors the existing migrations' SQLite-friendly portability: stick to TEXT
for UUIDs and JSON payloads so the same DDL applies to both SQLite and
Postgres deployments. (`PRIMARY KEY (library_id, uuid)` indexes the lookup
key; the secondary name index covers the `list_symbols` / `list_footprints`
/ `list_sims` UI surface that sorts alphabetically.)

## Source
Lines 12–20 in `crates/oxide-library-server/migrations/004_primitives.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [004_primitives](/crates/oxide-library-server/migrations/004_primitives.md) |
