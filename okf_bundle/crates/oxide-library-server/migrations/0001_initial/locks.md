---
okf_version: "0.2"
type: Table
title: locks
description: Table defined in 0001_initial.sql
resource: crates/oxide-library-server/migrations/0001_initial.sql
tags:
  - "lang:sql"
  - "type:Table"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0001_initial/locks
language: sql
---

# locks

Table defined in 0001_initial.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS locks (
    component_uuid TEXT NOT NULL,
    field_set      TEXT NOT NULL,
    holder         TEXT NOT NULL,
    acquired       TEXT NOT NULL,
    last_renewed   TEXT NOT N ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `component_uuid` | `TEXT` | `NOT NULL` |
| `field_set` | `TEXT` | `NOT NULL` |
| `holder` | `TEXT` | `NOT NULL` |
| `acquired` | `TEXT` | `NOT NULL` |
| `last_renewed` | `TEXT` | `NOT NULL` |

## Source
Lines 84–91 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
