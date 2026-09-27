---
okf_version: "0.2"
type: Function
title: list_symbols
resource: crates/oxide-library-server/src/routes/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:17Z"
concept_id: crates/oxide-library-server/src/routes/symbols/list_symbols
language: rust
---

# list_symbols

## Signature

```rust
fn list_symbols(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 60–66 in `crates/oxide-library-server/src/routes/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-library-server/src/routes/symbols.md) |
