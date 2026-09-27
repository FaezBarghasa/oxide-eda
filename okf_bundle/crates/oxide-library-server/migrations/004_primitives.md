---
okf_version: "0.2"
type: Module
title: 004_primitives
description: v0.9 library refactor (WS-C step C3) — primitive storage tables.
resource: crates/oxide-library-server/migrations/004_primitives.sql
tags:
  - "lang:sql"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library-server/migrations/004_primitives
language: sql
---

# 004_primitives

v0.9 library refactor (WS-C step C3) — primitive storage tables.

## Docstring

v0.9 library refactor (WS-C step C3) — primitive storage tables.

Reusable shape primitives (Symbol/Footprint/SimModel) addressed by a
(library_id, uuid) tuple per `v0.9-refactor-2-plan.md` §2 / §8.

Mirrors the existing migrations' SQLite-friendly portability: stick to TEXT
for UUIDs and JSON payloads so the same DDL applies to both SQLite and
Postgres deployments. (`PRIMARY KEY (library_id, uuid)` indexes the lookup
key; the secondary name index covers the `list_symbols` / `list_footprints`
/ `list_sims` UI surface that sorts alphabetically.)

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-library-server/migrations/004_primitives/symbols.md) |
| related | [idx_symbols_library_name](/crates/oxide-library-server/migrations/004_primitives/idx_symbols_library_name.md) |
| related | [footprints](/crates/oxide-library-server/migrations/004_primitives/footprints.md) |
| related | [idx_footprints_library_name](/crates/oxide-library-server/migrations/004_primitives/idx_footprints_library_name.md) |
| related | [sims](/crates/oxide-library-server/migrations/004_primitives/sims.md) |
| related | [idx_sims_library_name](/crates/oxide-library-server/migrations/004_primitives/idx_sims_library_name.md) |
