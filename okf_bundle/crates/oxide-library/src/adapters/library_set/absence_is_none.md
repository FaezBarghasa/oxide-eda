---
okf_version: "0.2"
type: Function
title: absence_is_none
description: "Fold an adapter lookup into the resolver's `Ok(None)` = \"not there\""
resource: crates/oxide-library/src/adapters/library_set.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/adapters/library_set/absence_is_none
language: rust
---

# absence_is_none

Fold an adapter lookup into the resolver's `Ok(None)` = "not there"

## Signature

```rust
fn absence_is_none(got: Result<T, LibraryError>) -> Result<Option<T>, LibraryError>
```

## Type Parameters

- `T`

## Docstring

Fold an adapter lookup into the resolver's `Ok(None)` = "not there"
/ `Err` = "could not tell" split.

## Source
Lines 325–331 in `crates/oxide-library/src/adapters/library_set.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_set](/crates/oxide-library/src/adapters/library_set.md) |
| called_by | [resolve_footprint](/crates/oxide-library/src/adapters/library_set/resolve_footprint.md) |
| called_by | [resolve_sim](/crates/oxide-library/src/adapters/library_set/resolve_sim.md) |
| called_by | [resolve_symbol](/crates/oxide-library/src/adapters/library_set/resolve_symbol.md) |
