---
okf_version: "0.2"
type: Index
title: idx_components_internal_pn
description: Index defined in 0001_initial.sql
resource: crates/oxide-library-server/migrations/0001_initial.sql
tags:
  - "lang:sql"
  - "type:Index"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0001_initial/idx_components_internal_pn
language: sql
---

# idx_components_internal_pn

Index defined in 0001_initial.sql

## Signature

```sql
CREATE INDEX IF NOT EXISTS idx_components_internal_pn ON components(internal_pn)
```

## Source
Lines 25–25 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
