---
okf_version: "0.2"
type: Function
title: delete_empty_table
description: "Delete the table `name` from the library. Adapters MUST refuse"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/delete_empty_table
language: rust
---

# delete_empty_table

Delete the table `name` from the library. Adapters MUST refuse

## Signature

```rust
fn delete_empty_table(&self, _name: &str, _msg: &str) -> Result<(), LibraryError>
```

## Docstring

Delete the table `name` from the library. Adapters MUST refuse
when the table still contains rows — Manage Tables uses this
surface and the user can drop rows individually first. Returns
`NotFound` when the table doesn't exist, `Conflict` when it's
non-empty.

## Source
Lines 221–225 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
