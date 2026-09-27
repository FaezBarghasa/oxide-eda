---
okf_version: "0.2"
type: Function
title: unreadable_documents_are_counted_and_the_first_error_survives
description: "The dropped rows are the whole finding: without a tally the"
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/unreadable_documents_are_counted_and_the_first_error_survives
language: rust
---

# unreadable_documents_are_counted_and_the_first_error_survives

The dropped rows are the whole finding: without a tally the

## Signature

```rust
fn unreadable_documents_are_counted_and_the_first_error_survives()
```

## Decorators

- `test`

## Docstring

The dropped rows are the whole finding: without a tally the
caller cannot tell "1 part matched" from "3 matched and 2 could
not be read", and the user concludes the missing parts are not
in the library and re-creates them.
[test]

## Source
Lines 816–830 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
| calls | [split_readable_docs](/crates/oxide-library/src/search_index/split_readable_docs.md) |
