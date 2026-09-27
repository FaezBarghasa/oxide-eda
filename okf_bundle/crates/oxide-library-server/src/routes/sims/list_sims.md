---
okf_version: "0.2"
type: Function
title: list_sims
resource: crates/oxide-library-server/src/routes/sims.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:29Z"
concept_id: crates/oxide-library-server/src/routes/sims/list_sims
language: rust
---

# list_sims

## Signature

```rust
fn list_sims(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 60–66 in `crates/oxide-library-server/src/routes/sims.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sims](/crates/oxide-library-server/src/routes/sims.md) |
