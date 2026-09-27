---
okf_version: "0.2"
type: Class
title: LibraryQuery
description: "A query into the library — partial match on internal_pn or mpn, plus facets."
resource: crates/oxide-library/src/adapter.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapter/LibraryQuery
language: rust
---

# LibraryQuery

A query into the library — partial match on internal_pn or mpn, plus facets.

## Signature

```rust
pub struct LibraryQuery
```

## Decorators

- `derive(Clone, Debug, Default)`

## Visibility

- `pub`

## Docstring

A query into the library — partial match on internal_pn or mpn, plus facets.
[derive(Clone, Debug, Default)]

## Methods

- `text`
- `category`
- `facets`

## Source
Lines 52–56 in `crates/oxide-library/src/adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [adapter](/crates/oxide-library/src/adapter.md) |
