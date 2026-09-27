---
okf_version: "0.2"
type: Function
title: new_sqlite_memory
description: "Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`."
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/new_sqlite_memory
language: rust
---

# new_sqlite_memory

Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`.

## Signature

```rust
impl AppState { pub fn new_sqlite_memory() -> Result<Self, ApiError> }
```

## Visibility

- `pub`

## Docstring

Alias for backwards compatibility with tests and callers expecting `new_sqlite_memory`.

## Source
Lines 86–88 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
