---
okf_version: "0.2"
type: Function
title: list_rows_in_table
resource: crates/oxide-library-server/src/routes/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:04Z"
concept_id: crates/oxide-library-server/src/routes/tables/list_rows_in_table
language: rust
---

# list_rows_in_table

## Signature

```rust
fn list_rows_in_table(
    state: web::Data<AppState>,
    name: web::Path<String>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 48–55 in `crates/oxide-library-server/src/routes/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library-server/src/routes/tables.md) |
