---
okf_version: "0.2"
type: Function
title: delete_row
resource: crates/oxide-library-server/src/routes/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:11Z"
concept_id: crates/oxide-library-server/src/routes/rows/delete_row
language: rust
---

# delete_row

## Signature

```rust
fn delete_row(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 98–110 in `crates/oxide-library-server/src/routes/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-library-server/src/routes/rows.md) |
| calls | [parse_row_id](/crates/oxide-library-server/src/routes/rows/parse_row_id.md) |
