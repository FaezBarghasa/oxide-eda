---
okf_version: "0.2"
type: Function
title: unparseable_query_text_matches_literally_instead_of_returning_the_library
description: "Tantivy reads `( ) + - : \" [ ] ^ *` as operators, and real part"
resource: crates/oxide-library/tests/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/tests/search_index/unparseable_query_text_matches_literally_instead_of_returning_the_library
language: rust
---

# unparseable_query_text_matches_literally_instead_of_returning_the_library

Tantivy reads `( ) + - : " [ ] ^ *` as operators, and real part

## Signature

```rust
fn unparseable_query_text_matches_literally_instead_of_returning_the_library()
```

## Decorators

- `test`

## Docstring

Tantivy reads `( ) + - : " [ ] ^ *` as operators, and real part
numbers are full of them. A query that fails to parse used to be
replaced with `AllQuery`, handing back the entire library dressed
up as matches. The fallback now matches the same text literally.
[test]

## Source
Lines 394–430 in `crates/oxide-library/tests/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/tests/search_index.md) |
| calls | [serial_guard](/crates/oxide-library/tests/search_index/serial_guard.md) |
| calls | [fixture_corpus](/crates/oxide-library/tests/search_index/fixture_corpus.md) |
