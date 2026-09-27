---
okf_version: "0.2"
type: Function
title: where_used_for
description: Look up the use-sites for a row.
resource: crates/oxide-app/src/library/state/methods.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/methods/where_used_for
language: rust
---

# where_used_for

Look up the use-sites for a row.

## Signature

```rust
impl LibraryState { pub fn where_used_for(&self, row_id: RowId) -> Vec<UseSite> }
```

## Visibility

- `pub`

## Docstring

Look up the use-sites for a row.

## Source
Lines 291–293 in `crates/oxide-app/src/library/state/methods.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [methods](/crates/oxide-app/src/library/state/methods.md) |
