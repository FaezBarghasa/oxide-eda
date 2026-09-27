---
okf_version: "0.2"
type: Table
title: review_requests
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
concept_id: crates/oxide-library-server/migrations/0001_initial/review_requests
language: sql
---

# review_requests

Table defined in 0001_initial.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS review_requests (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    component_uuid TEXT NOT NULL,
    major          INTEGER NOT NULL,
    minor          INTEGER NOT ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `id` | `INTEGER` | `PRIMARY KEY` |
| `component_uuid` | `TEXT` | `NOT NULL` |
| `major` | `INTEGER` | `NOT NULL` |
| `minor` | `INTEGER` | `NOT NULL` |
| `submitter` | `TEXT` | `NOT NULL` |
| `submitted` | `TEXT` | `NOT NULL` |
| `state` | `TEXT` | `NOT NULL` |
| `reviewer` | `TEXT` | `` |
| `decided` | `TEXT` | `` |
| `note` | `TEXT` | `` |

## Source
Lines 93–106 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
