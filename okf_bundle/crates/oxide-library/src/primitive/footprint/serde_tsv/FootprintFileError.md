---
okf_version: "0.2"
type: Class
title: FootprintFileError
description: "Error variants raised by [`FootprintFile`] parsers + serialisers."
resource: crates/oxide-library/src/primitive/footprint/serde_tsv.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/serde_tsv/FootprintFileError
language: rust
---

# FootprintFileError

Error variants raised by [`FootprintFile`] parsers + serialisers.

## Signature

```rust
pub enum FootprintFileError
```

## Decorators

- `derive(Debug, thiserror::Error)`

## Visibility

- `pub`

## Docstring

Error variants raised by [`FootprintFile`] parsers + serialisers.
[derive(Debug, thiserror::Error)]

## Methods

- `got`
- `column`
- `value`
- `got`
- `row_index`
- `got`
- `expected`
- `kind`
- `got`
- `column`
- `value`

## Source
Lines 314–346 in `crates/oxide-library/src/primitive/footprint/serde_tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [serde_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv.md) |
