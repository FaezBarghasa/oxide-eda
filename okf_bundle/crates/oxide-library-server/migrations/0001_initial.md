---
okf_version: "0.2"
type: Module
title: 0001_initial
description: Oxide library DB-flavour initial schema.
resource: crates/oxide-library-server/migrations/0001_initial.sql
tags:
  - "lang:sql"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library-server/migrations/0001_initial
language: sql
---

# 0001_initial

Oxide library DB-flavour initial schema.

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

## Relationships

| Type | Target |
|------|--------|
| related | [components](/crates/oxide-library-server/migrations/0001_initial/components.md) |
| related | [idx_components_internal_pn](/crates/oxide-library-server/migrations/0001_initial/idx_components_internal_pn.md) |
| related | [revisions](/crates/oxide-library-server/migrations/0001_initial/revisions.md) |
| related | [idx_revisions_state](/crates/oxide-library-server/migrations/0001_initial/idx_revisions_state.md) |
| related | [parameters](/crates/oxide-library-server/migrations/0001_initial/parameters.md) |
| related | [idx_parameters_key](/crates/oxide-library-server/migrations/0001_initial/idx_parameters_key.md) |
| related | [suppliers](/crates/oxide-library-server/migrations/0001_initial/suppliers.md) |
| related | [idx_suppliers_distributor_sku](/crates/oxide-library-server/migrations/0001_initial/idx_suppliers_distributor_sku.md) |
| related | [lifecycle_log](/crates/oxide-library-server/migrations/0001_initial/lifecycle_log.md) |
| related | [idx_lifecycle_log_component](/crates/oxide-library-server/migrations/0001_initial/idx_lifecycle_log_component.md) |
| related | [locks](/crates/oxide-library-server/migrations/0001_initial/locks.md) |
| related | [review_requests](/crates/oxide-library-server/migrations/0001_initial/review_requests.md) |
| related | [idx_review_requests_state](/crates/oxide-library-server/migrations/0001_initial/idx_review_requests_state.md) |
