---
okf_version: "0.2"
type: Function
title: acquire_lock
resource: crates/oxide-library-server/src/routes/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:36Z"
concept_id: crates/oxide-library-server/src/routes/locks/acquire_lock
language: rust
---

# acquire_lock

## Signature

```rust
fn acquire_lock(
    state: web::Data<AppState>,
    row_id: web::Path<String>,
    req: HttpRequest,
    body: web::Json<LockBody>,
) -> Result<HttpResponse, ApiError>
```

## Source
Lines 75–94 in `crates/oxide-library-server/src/routes/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/routes/locks.md) |
| calls | [holder_from](/crates/oxide-library-server/src/routes/locks/holder_from.md) |
