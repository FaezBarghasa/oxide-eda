---
okf_version: "0.2"
type: Function
title: create_row
resource: crates/oxide-library-server/src/routes/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:11Z"
concept_id: crates/oxide-library-server/src/routes/rows/create_row
language: rust
---

# create_row

## Signature

```rust
fn create_row(
    state: web::Data<AppState>,
    name: web::Path<String>,
    q: web::Query<LibraryQuery>,
    row: web::Json<ComponentRow>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 43–58 in `crates/oxide-library-server/src/routes/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-library-server/src/routes/rows.md) |
