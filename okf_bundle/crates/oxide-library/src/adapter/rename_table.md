---
okf_version: "0.2"
type: Function
title: rename_table
description: "Rename a table — `old` → `new`, preserving every row inside."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/rename_table
language: rust
---

# rename_table

Rename a table — `old` → `new`, preserving every row inside.

## Signature

```rust
fn rename_table(&self, _old: &str, _new: &str, _msg: &str) -> Result<(), LibraryError>
```

## Docstring

Rename a table — `old` → `new`, preserving every row inside.
Returns `NotFound` when `old` doesn't exist, `Conflict` when
`new` already does (no silent overwrite). The new name is
validated for filename-safe characters at the adapter
boundary.

## Source
Lines 232–236 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
