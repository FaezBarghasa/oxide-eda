---
okf_version: "0.2"
type: Function
title: new_memory
description: Open an in-memory SurrealDB database.
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/new_memory_1
language: rust
---

# new_memory

Open an in-memory SurrealDB database.

## Signature

```rust
pub fn new_memory() -> Result<Self, ApiError>
```

## Visibility

- `pub`

## Docstring

Open an in-memory SurrealDB database.

## Source
Lines 68–83 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
