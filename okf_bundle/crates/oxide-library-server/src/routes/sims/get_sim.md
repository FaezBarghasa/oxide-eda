---
okf_version: "0.2"
type: Function
title: get_sim
resource: crates/oxide-library-server/src/routes/sims.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:29Z"
concept_id: crates/oxide-library-server/src/routes/sims/get_sim
language: rust
---

# get_sim

## Signature

```rust
fn get_sim(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 44–58 in `crates/oxide-library-server/src/routes/sims.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sims](/crates/oxide-library-server/src/routes/sims.md) |
