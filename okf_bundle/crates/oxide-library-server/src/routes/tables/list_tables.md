---
okf_version: "0.2"
type: Function
title: list_tables
resource: crates/oxide-library-server/src/routes/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:04Z"
concept_id: crates/oxide-library-server/src/routes/tables/list_tables
language: rust
---

# list_tables

## Signature

```rust
fn list_tables(
    state: web::Data<AppState>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 40–46 in `crates/oxide-library-server/src/routes/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library-server/src/routes/tables.md) |
