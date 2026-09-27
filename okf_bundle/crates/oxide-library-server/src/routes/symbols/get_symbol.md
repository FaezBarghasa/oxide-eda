---
okf_version: "0.2"
type: Function
title: get_symbol
resource: crates/oxide-library-server/src/routes/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:17Z"
concept_id: crates/oxide-library-server/src/routes/symbols/get_symbol
language: rust
---

# get_symbol

## Signature

```rust
fn get_symbol(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 44–58 in `crates/oxide-library-server/src/routes/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-library-server/src/routes/symbols.md) |
