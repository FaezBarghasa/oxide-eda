---
okf_version: "0.2"
type: Function
title: create_footprint
resource: crates/oxide-library-server/src/routes/footprints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:23Z"
concept_id: crates/oxide-library-server/src/routes/footprints/create_footprint
language: rust
---

# create_footprint

## Signature

```rust
fn create_footprint(
    state: web::Data<AppState>,
    body: web::Json<CreateBody>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 35–44 in `crates/oxide-library-server/src/routes/footprints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprints](/crates/oxide-library-server/src/routes/footprints.md) |
