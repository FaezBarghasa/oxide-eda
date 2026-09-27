---
okf_version: "0.2"
type: Function
title: unparseable_query_text_with_no_match_returns_nothing
description: A query whose text survives quoting but matches nothing must come
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index/unparseable_query_text_with_no_match_returns_nothing
language: rust
---

# unparseable_query_text_with_no_match_returns_nothing

A query whose text survives quoting but matches nothing must come

## Signature

```rust
fn unparseable_query_text_with_no_match_returns_nothing()
```

## Decorators

- `test`

## Docstring

A query whose text survives quoting but matches nothing must come
back empty, not widened into the whole library.
[test]

## Source
Lines 435–455 in `crates/oxide-library/tests/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/tests/search_index.md) |
| calls | [serial_guard](/crates/oxide-library/tests/search_index/serial_guard.md) |
| calls | [fixture_corpus](/crates/oxide-library/tests/search_index/fixture_corpus.md) |
