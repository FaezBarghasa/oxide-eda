---
okf_version: "0.2"
type: Function
title: create_empty_table
description: "Create an empty table named `name`. Used by the New Component"
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/create_empty_table
language: rust
---

# create_empty_table

Create an empty table named `name`. Used by the New Component

## Signature

```rust
fn create_empty_table(&self, _name: &str, _msg: &str) -> Result<(), LibraryError>
```

## Docstring

Create an empty table named `name`. Used by the New Component
modal's `+ New Table…` flow so users can mint destination
tables without dropping into the manifest by hand. Adapters
that don't support manifest mutation surface a `Backend`
error; the UI should disable the create flow when this fails.
`msg` is the commit message for storage backends that require
one (LocalGit). Returns `Conflict` when a table with the same
name already exists.

## Source
Lines 210–214 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
