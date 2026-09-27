---
okf_version: "0.2"
type: Function
title: unauthorized
resource: crates/oxide-library-server/src/routes/error.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T09:46:19Z"
concept_id: crates/oxide-library-server/src/routes/error/unauthorized
language: rust
---

# unauthorized

## Signature

```rust
impl ApiError { pub fn unauthorized(msg: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 44–49 in `crates/oxide-library-server/src/routes/error.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [error](/crates/oxide-library-server/src/routes/error.md) |
