---
okf_version: "0.2"
type: Function
title: create_symbol
resource: crates/oxide-library-server/src/routes/symbols.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:17Z"
concept_id: crates/oxide-library-server/src/routes/symbols/create_symbol
language: rust
---

# create_symbol

## Signature

```rust
fn create_symbol(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 35–42 in `crates/oxide-library-server/src/routes/symbols.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbols](/crates/oxide-library-server/src/routes/symbols.md) |
