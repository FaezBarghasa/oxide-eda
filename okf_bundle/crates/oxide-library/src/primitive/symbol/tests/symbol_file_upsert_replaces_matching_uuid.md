---
okf_version: "0.2"
type: Function
title: symbol_file_upsert_replaces_matching_uuid
description: "`SymbolFile::upsert` replaces a matching-uuid symbol in-place"
resource: crates/oxide-library/src/primitive/symbol/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/symbol/tests/symbol_file_upsert_replaces_matching_uuid
language: rust
---

# symbol_file_upsert_replaces_matching_uuid

`SymbolFile::upsert` replaces a matching-uuid symbol in-place

## Signature

```rust
fn symbol_file_upsert_replaces_matching_uuid()
```

## Decorators

- `test`

## Docstring

`SymbolFile::upsert` replaces a matching-uuid symbol in-place
and returns true; non-matching uuids return false so the
caller can `push` instead.
[test]

## Source
Lines 66–79 in `crates/oxide-library/src/primitive/symbol/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/symbol/tests.md) |
