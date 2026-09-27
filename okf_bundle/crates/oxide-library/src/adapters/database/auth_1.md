---
okf_version: "0.2"
type: Function
title: auth
description: "Apply the `Authorization: Bearer <token>` header when configured."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/auth_1
language: rust
---

# auth

Apply the `Authorization: Bearer <token>` header when configured.

## Signature

```rust
fn auth(&self, req: reqwest::blocking::RequestBuilder) -> reqwest::blocking::RequestBuilder
```

## Docstring

Apply the `Authorization: Bearer <token>` header when configured.

## Source
Lines 180–186 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
