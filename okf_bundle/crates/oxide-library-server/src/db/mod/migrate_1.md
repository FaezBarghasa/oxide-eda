---
okf_version: "0.2"
type: Function
title: migrate
description: Apply schema definitions to SurrealDB.
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/migrate_1
language: rust
---

# migrate

Apply schema definitions to SurrealDB.

## Signature

```rust
pub fn migrate(&self) -> Result<(), ApiError>
```

## Visibility

- `pub`

## Docstring

Apply schema definitions to SurrealDB.

## Source
Lines 111–136 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
