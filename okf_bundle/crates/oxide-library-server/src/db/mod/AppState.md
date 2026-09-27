---
okf_version: "0.2"
type: Class
title: AppState
description: Server-side state shared across all actix handlers.
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/AppState
language: rust
---

# AppState

Server-side state shared across all actix handlers.

## Signature

```rust
pub struct AppState
```

## Decorators

- `derive(Clone)`

## Visibility

- `pub`

## Docstring

Server-side state shared across all actix handlers.
[derive(Clone)]

## Methods

- `db`
- `locks`

## Source
Lines 61–64 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
