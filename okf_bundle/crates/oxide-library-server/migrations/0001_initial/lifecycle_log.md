---
okf_version: "0.2"
type: Table
title: lifecycle_log
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
concept_id: crates/oxide-library-server/migrations/0001_initial/lifecycle_log
language: sql
---

# lifecycle_log

Table defined in 0001_initial.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS lifecycle_log (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    component_uuid TEXT NOT NULL,
    major          INTEGER NOT NULL,
    minor          INTEGER NOT N ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `id` | `INTEGER` | `PRIMARY KEY` |
| `component_uuid` | `TEXT` | `NOT NULL` |
| `major` | `INTEGER` | `NOT NULL` |
| `minor` | `INTEGER` | `NOT NULL` |
| `from_state` | `TEXT` | `` |
| `to_state` | `TEXT` | `NOT NULL` |
| `actor` | `TEXT` | `NOT NULL` |
| `occurred` | `TEXT` | `NOT NULL` |
| `note` | `TEXT` | `` |

## Source
Lines 70–80 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
