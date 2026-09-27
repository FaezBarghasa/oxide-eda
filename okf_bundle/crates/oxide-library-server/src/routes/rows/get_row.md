---
okf_version: "0.2"
type: Function
title: get_row
resource: crates/oxide-library-server/src/routes/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:11Z"
concept_id: crates/oxide-library-server/src/routes/rows/get_row
language: rust
---

# get_row

## Signature

```rust
fn get_row(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    q: web::Query<LibraryQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 60–72 in `crates/oxide-library-server/src/routes/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-library-server/src/routes/rows.md) |
| calls | [parse_row_id](/crates/oxide-library-server/src/routes/rows/parse_row_id.md) |
