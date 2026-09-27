---
okf_version: "0.2"
type: Table
title: revisions
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
concept_id: crates/oxide-library-server/migrations/0001_initial/revisions
language: sql
---

# revisions

Table defined in 0001_initial.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS revisions (
    component_uuid TEXT NOT NULL,
    major          INTEGER NOT NULL,
    minor          INTEGER NOT NULL,
    state          TEXT NOT NULL,
    author          ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `component_uuid` | `TEXT` | `NOT NULL` |
| `major` | `INTEGER` | `NOT NULL` |
| `minor` | `INTEGER` | `NOT NULL` |
| `state` | `TEXT` | `NOT NULL` |
| `author` | `TEXT` | `NOT NULL` |
| `message` | `TEXT` | `NOT NULL` |
| `created` | `TEXT` | `NOT NULL` |
| `content_hash` | `TEXT` | `NOT NULL` |
| `payload` | `TEXT` | `NOT NULL` |

## Source
Lines 27–39 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
