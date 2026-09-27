---
okf_version: "0.2"
type: Table
title: components
description: Oxide library DB-flavour initial schema.
resource: crates/oxide-library-server/migrations/0001_initial.sql
tags:
  - "lang:sql"
  - "type:Table"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0001_initial/components
language: sql
---

# components

Oxide library DB-flavour initial schema.

## Signature

```sql
CREATE TABLE IF NOT EXISTS components (
    uuid           TEXT PRIMARY KEY NOT NULL,
    internal_pn    TEXT NOT NULL,
    head_major     INTEGER NOT NULL,
    head_minor     INTEGER NOT NULL,
    cr ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `uuid` | `TEXT` | `PRIMARY KEY NOT NULL` |
| `internal_pn` | `TEXT` | `NOT NULL` |
| `head_major` | `INTEGER` | `NOT NULL` |
| `head_minor` | `INTEGER` | `NOT NULL` |
| `created` | `TEXT` | `NOT NULL` |
| `updated` | `TEXT` | `NOT NULL` |

## Docstring

Oxide library DB-flavour initial schema.

Tables (per v0.9-library-plan.md §7 + WS-B contract):
components        — one row per logical component (uuid, internal_pn, head version)
revisions         — one row per (component_uuid, version) — full revision JSON blob
parameters        — flattened (component_uuid, version, key) for fast facet queries
suppliers         — flattened supplier links per revision
lifecycle_log     — append-only audit of state transitions
locks             — advisory locks per (uuid, field_set) with idle TTL
review_requests   — submitted-but-not-yet-released revisions awaiting reviewer signoff

Schema is portable between SQLite and Postgres: text + integer + timestamp text.
Postgres-specific types (uuid, jsonb) are intentionally NOT used here so the
same migration applies to both backends.

## Source
Lines 16–23 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
