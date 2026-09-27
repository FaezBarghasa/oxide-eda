---
okf_version: "0.2"
type: Function
title: connect
description: Connect to a database backend. Supports memory and remote SurrealDB instances.
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/connect
language: rust
---

# connect

Connect to a database backend. Supports memory and remote SurrealDB instances.

## Signature

```rust
impl AppState { pub fn connect(_url: &str) -> Result<Self, ApiError> }
```

## Visibility

- `pub`

## Docstring

Connect to a database backend. Supports memory and remote SurrealDB instances.

## Source
Lines 91–94 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
