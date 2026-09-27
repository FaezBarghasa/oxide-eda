---
okf_version: "0.2"
type: Function
title: create_sim
resource: crates/oxide-library-server/src/routes/sims.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:29Z"
concept_id: crates/oxide-library-server/src/routes/sims/create_sim
language: rust
---

# create_sim

## Signature

```rust
fn create_sim(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 35–42 in `crates/oxide-library-server/src/routes/sims.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sims](/crates/oxide-library-server/src/routes/sims.md) |
