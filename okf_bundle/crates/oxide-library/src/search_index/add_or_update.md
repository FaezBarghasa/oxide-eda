---
okf_version: "0.2"
type: Function
title: add_or_update
description: Add or replace the doc for a single row.
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/add_or_update
language: rust
---

# add_or_update

Add or replace the doc for a single row.

## Signature

```rust
impl TantivySearchIndex { pub fn add_or_update(&self, row: &ComponentRow) -> Result<(), TantivyIndexError> }
```

## Visibility

- `pub`

## Docstring

Add or replace the doc for a single row.

Full Tantivy rewiring for the DBLib model is deferred (see
`v0.9-refactor-2-plan.md` §17). This entry mirrors the original
schema but reads from a [`ComponentRow`] directly instead of
walking a `Revision` chain. The `head_major` / `head_minor`
columns are filled with zeros and survive only so the on-disk
schema stays compatible until a future polish pass drops them.

Replacement is by `row_id` term; safe to call on the same row
repeatedly. Caller must `commit()` to make changes visible to
subsequent queries.

## Source
Lines 296–363 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| calls | [add_text](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_text.md) |
| calls | [lifecycle_token](/crates/oxide-library/src/search_index/lifecycle_token.md) |
