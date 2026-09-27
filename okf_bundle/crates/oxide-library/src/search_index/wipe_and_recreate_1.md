---
okf_version: "0.2"
type: Function
title: wipe_and_recreate
description: "M10: nuke the on-disk index at `path` and re-open with the current"
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/wipe_and_recreate_1
language: rust
---

# wipe_and_recreate

M10: nuke the on-disk index at `path` and re-open with the current

## Signature

```rust
pub fn wipe_and_recreate(path: impl AsRef<Path>) -> Result<Self, TantivyIndexError>
```

## Visibility

- `pub`

## Docstring

M10: nuke the on-disk index at `path` and re-open with the current
schema. The canonical recovery flow when [`TantivyIndexError::SchemaMismatch`]
is returned by [`TantivySearchIndex::open`]:

```ignore
match TantivySearchIndex::open(&p) {
Ok(idx) => idx,
Err(TantivyIndexError::SchemaMismatch { .. }) => {
TantivySearchIndex::wipe_and_recreate(&p)?
// caller must re-ingest every component
}
Err(e) => return Err(e.into()),
}
```

Wipes only the directory contents — the directory itself is reused.
Callers are responsible for re-populating the index from the canonical
component store; this helper does **not** rebuild documents.

## Source
Lines 256–273 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
