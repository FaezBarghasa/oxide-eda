---
okf_version: "0.2"
type: Index
title: idx_suppliers_distributor_sku
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
concept_id: crates/oxide-library-server/migrations/0001_initial/idx_suppliers_distributor_sku
language: sql
---

# idx_suppliers_distributor_sku

Index defined in 0001_initial.sql

## Signature

```sql
CREATE INDEX IF NOT EXISTS idx_suppliers_distributor_sku ON suppliers(distributor, sku)
```

## Source
Lines 68–68 in `crates/oxide-library-server/migrations/0001_initial.sql`

## Relationships

| Type | Target |
|------|--------|
| related | [0001_initial](/crates/oxide-library-server/migrations/0001_initial.md) |
