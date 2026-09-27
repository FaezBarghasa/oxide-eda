---
okf_version: "0.2"
type: Function
title: open
description: "Open or create a Tantivy index rooted at `path`."
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/open
language: rust
---

# open

Open or create a Tantivy index rooted at `path`.

## Signature

```rust
impl TantivySearchIndex { pub fn open(path: impl AsRef<Path>) -> Result<Self, TantivyIndexError> }
```

## Visibility

- `pub`

## Docstring

Open or create a Tantivy index rooted at `path`.

- If `path` already contains a Tantivy index, it is reopened.
- Otherwise the directory is created and a fresh index is initialised
with the schema described in the module docs.
- If an existing index has a different schema than the current
[`NUMERIC_PARAM_KEYS`] would produce, [`TantivyIndexError::SchemaMismatch`]
is returned.

## Source
Lines 204–236 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
