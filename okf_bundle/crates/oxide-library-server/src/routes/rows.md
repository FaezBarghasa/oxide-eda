---
okf_version: "0.2"
type: Module
title: rows
description: "`/tables/:name/rows` routes — per-row CRUD over `ComponentRow`."
resource: crates/oxide-library-server/src/routes/rows.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:11Z"
concept_id: crates/oxide-library-server/src/routes/rows
language: rust
---

# rows

`/tables/:name/rows` routes — per-row CRUD over `ComponentRow`.

## Docstring

`/tables/:name/rows` routes — per-row CRUD over `ComponentRow`.

All routes carry `?library_id=<uuid>` to scope into one library
inside the shared `component_rows` table. JSON body shape is
`ComponentRow` directly — no envelope wrapper.

```text
POST   /tables/:name/rows             insert row, body=ComponentRow
GET    /tables/:name/rows/:row_id     fetch one row
PUT    /tables/:name/rows/:row_id     replace, body=ComponentRow
DELETE /tables/:name/rows/:row_id     delete, 204 on success
```

`:row_id` is parsed as a [`RowId`] — a UUIDv7 newtype.

## Relationships

| Type | Target |
|------|--------|
| related | [configure](/crates/oxide-library-server/src/routes/rows/configure.md) |
| related | [LibraryQuery](/crates/oxide-library-server/src/routes/rows/LibraryQuery.md) |
| related | [create_row](/crates/oxide-library-server/src/routes/rows/create_row.md) |
| related | [get_row](/crates/oxide-library-server/src/routes/rows/get_row.md) |
| related | [update_row](/crates/oxide-library-server/src/routes/rows/update_row.md) |
| related | [delete_row](/crates/oxide-library-server/src/routes/rows/delete_row.md) |
| related | [parse_row_id](/crates/oxide-library-server/src/routes/rows/parse_row_id.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
