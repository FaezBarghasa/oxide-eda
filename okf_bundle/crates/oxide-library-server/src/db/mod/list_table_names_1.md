---
okf_version: "0.2"
type: Function
title: list_table_names
description: List the names of every distinct table that has at least one row
resource: crates/oxide-library-server/src/db/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library-server"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:44:57Z"
concept_id: crates/oxide-library-server/src/db/mod/list_table_names_1
language: rust
---

# list_table_names

List the names of every distinct table that has at least one row

## Signature

```rust
pub fn list_table_names(&self, library_id: Uuid) -> Result<Vec<String>, ApiError>
```

## Visibility

- `pub`

## Docstring

List the names of every distinct table that has at least one row
inside `library_id`.

## Source
Lines 267–285 in `crates/oxide-library-server/src/db/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [db](/crates/oxide-library-server/src/db/mod.md) |
