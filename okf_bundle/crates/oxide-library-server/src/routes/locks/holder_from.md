---
okf_version: "0.2"
type: Function
title: holder_from
resource: crates/oxide-library-server/src/routes/locks.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:34:36Z"
concept_id: crates/oxide-library-server/src/routes/locks/holder_from
language: rust
---

# holder_from

## Signature

```rust
fn holder_from(req: &HttpRequest) -> Result<String, ApiError>
```

## Source
Lines 53–73 in `crates/oxide-library-server/src/routes/locks.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [locks](/crates/oxide-library-server/src/routes/locks.md) |
| called_by | [acquire_lock](/crates/oxide-library-server/src/routes/locks/acquire_lock.md) |
| called_by | [release_lock](/crates/oxide-library-server/src/routes/locks/release_lock.md) |
