---
okf_version: "0.2"
type: Function
title: param_on_disk
description: "The parameter as it now stands on disk, read back through a fresh"
resource: crates/oxide-app/tests/regression/library_browser_cell_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_browser_cell_commit/param_on_disk
language: rust
---

# param_on_disk

The parameter as it now stands on disk, read back through a fresh

## Signature

```rust
fn param_on_disk(snxlib: &std::path::Path, row_id: RowId) -> ParamValue
```

## Docstring

The parameter as it now stands on disk, read back through a fresh
adapter rather than the app's in-memory cache.

## Source
Lines 109–118 in `crates/oxide-app/tests/regression/library_browser_cell_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser_cell_commit](/crates/oxide-app/tests/regression/library_browser_cell_commit.md) |
