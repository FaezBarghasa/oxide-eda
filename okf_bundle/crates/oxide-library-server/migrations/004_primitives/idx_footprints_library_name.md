---
okf_version: "0.2"
type: Index
title: idx_footprints_library_name
description: Index defined in 004_primitives.sql
resource: crates/oxide-library-server/migrations/004_primitives.sql
tags:
  - "lang:sql"
  - "type:Index"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library-server/migrations/004_primitives/idx_footprints_library_name
language: sql
---

# idx_footprints_library_name

Index defined in 004_primitives.sql

## Signature

```sql
CREATE INDEX IF NOT EXISTS idx_footprints_library_name ON footprints(library_id, name)
```

## Source
Lines 32–32 in `crates/oxide-library-server/migrations/004_primitives.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [004_primitives](/crates/oxide-library-server/migrations/004_primitives.md) |
