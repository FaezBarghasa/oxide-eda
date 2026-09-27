---
okf_version: "0.2"
type: Function
title: list_footprints
resource: crates/oxide-library-server/src/routes/footprints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:23Z"
concept_id: crates/oxide-library-server/src/routes/footprints/list_footprints
language: rust
---

# list_footprints

## Signature

```rust
fn list_footprints(
    state: web::Data<AppState>,
    q: web::Query<ListQuery>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 62–68 in `crates/oxide-library-server/src/routes/footprints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprints](/crates/oxide-library-server/src/routes/footprints.md) |
