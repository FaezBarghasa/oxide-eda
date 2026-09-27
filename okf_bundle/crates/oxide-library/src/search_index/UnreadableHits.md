---
okf_version: "0.2"
type: Class
title: UnreadableHits
description: Tally of search hits whose stored document could not be read back
resource: crates/oxide-library/src/search_index.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T20:39:04Z"
concept_id: crates/oxide-library/src/search_index/UnreadableHits
language: rust
---

# UnreadableHits

Tally of search hits whose stored document could not be read back

## Signature

```rust
struct UnreadableHits
```

## Decorators

- `derive(Debug, Default, PartialEq, Eq)`

## Docstring

Tally of search hits whose stored document could not be read back
out of the index.
[derive(Debug, Default, PartialEq, Eq)]

## Methods

- `count`
- `first_error`

## Source
Lines 694–700 in `crates/oxide-library/src/search_index.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [search_index](/crates/oxide-library/src/search_index.md) |
