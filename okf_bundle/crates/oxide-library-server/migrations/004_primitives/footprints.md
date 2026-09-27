---
okf_version: "0.2"
type: Table
title: footprints
description: Table defined in 004_primitives.sql
resource: crates/oxide-library-server/migrations/004_primitives.sql
tags:
  - "lang:sql"
  - "type:Table"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library-server/migrations/004_primitives/footprints
language: sql
---

# footprints

Table defined in 004_primitives.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS footprints (
    library_id     TEXT NOT NULL,
    uuid           TEXT NOT NULL,
    name           TEXT NOT NULL,
    payload        TEXT NOT NULL,           -- full Footpr ...
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

## Source
Lines 23–31 in `crates/oxide-library-server/migrations/004_primitives.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [004_primitives](/crates/oxide-library-server/migrations/004_primitives.md) |
