---
okf_version: "0.2"
type: Function
title: read_table
description: Read every row from the named table.
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/read_table
language: rust
---

# read_table

Read every row from the named table.

## Signature

```rust
fn read_table(&self, _name: &str) -> Result<Vec<ComponentRow>, LibraryError>
```

## Docstring

Read every row from the named table.

## Source
Lines 346–350 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
