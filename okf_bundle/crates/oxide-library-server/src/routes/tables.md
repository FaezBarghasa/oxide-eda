---
okf_version: "0.2"
type: Module
title: tables
description: "`/tables` routes — DBLib row model."
resource: crates/oxide-library-server/src/routes/tables.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:04Z"
concept_id: crates/oxide-library-server/src/routes/tables
language: rust
---

# tables

`/tables` routes — DBLib row model.

## Docstring

`/tables` routes — DBLib row model.

Two endpoints under this prefix:

* `GET  /tables                 ?library_id=<uuid>`
→ `[String]` — distinct table names with at least one row.
* `GET  /tables/:name           ?library_id=<uuid>`
→ `[ComponentRow]` — every row in `name`, ordered by `internal_pn`.

`library_id` rides on the query string the same way it does for the
primitive routes (`/symbols` / `/footprints` / `/sims`). Once OIDC lands
in v0.9.x the value can be derived from the bearer token claims; until
then it stays explicit so the wire contract is symmetric across the
refactor.

## Relationships

| Type | Target |
|------|--------|
| related | [configure](/crates/oxide-library-server/src/routes/tables/configure.md) |
| related | [LibraryQuery](/crates/oxide-library-server/src/routes/tables/LibraryQuery.md) |
| related | [list_tables](/crates/oxide-library-server/src/routes/tables/list_tables.md) |
| related | [list_rows_in_table](/crates/oxide-library-server/src/routes/tables/list_rows_in_table.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
