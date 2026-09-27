---
okf_version: "0.2"
type: Function
title: get_footprint
resource: crates/oxide-library-server/src/routes/footprints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:23Z"
concept_id: crates/oxide-library-server/src/routes/footprints/get_footprint
language: rust
---

# get_footprint

## Signature

```rust
fn get_footprint(
    state: web::Data<AppState>,
    uuid: web::Path<String>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 46–60 in `crates/oxide-library-server/src/routes/footprints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprints](/crates/oxide-library-server/src/routes/footprints.md) |
