---
okf_version: "0.2"
type: Function
title: list_tables
description: "List the names of every table this library exposes (filename stem,"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/list_tables
language: rust
---

# list_tables

List the names of every table this library exposes (filename stem,

## Signature

```rust
fn list_tables(&self) -> Result<Vec<String>, LibraryError>
```

## Docstring

List the names of every table this library exposes (filename stem,
no extension).

## Source
Lines 196–200 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
