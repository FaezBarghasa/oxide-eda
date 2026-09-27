---
okf_version: "0.2"
type: Table
title: suppliers
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
concept_id: crates/oxide-library-server/migrations/0001_initial/suppliers
language: sql
---

# suppliers

Table defined in 0001_initial.sql

## Signature

```sql
CREATE TABLE IF NOT EXISTS suppliers (
    component_uuid TEXT NOT NULL,
    major          INTEGER NOT NULL,
    minor          INTEGER NOT NULL,
    distributor    TEXT NOT NULL,
    sku             ...
```

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `component_uuid` | `TEXT` | `NOT NULL` |
| `major` | `INTEGER` | `NOT NULL` |
| `minor` | `INTEGER` | `NOT NULL` |
| `distributor` | `TEXT` | `NOT NULL` |
| `sku` | `TEXT` | `NOT NULL` |

## Source
Lines 57–66 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
