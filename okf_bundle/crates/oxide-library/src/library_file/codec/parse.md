---
okf_version: "0.2"
type: Function
title: parse
description: "Parse a `.snxlib` TOML document. Validates the format token, then"
resource: crates/oxide-library/src/library_file/codec.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/codec/parse
language: rust
---

# parse

Parse a `.snxlib` TOML document. Validates the format token, then

## Signature

```rust
impl LibraryFile { pub fn parse(text: &str) -> Result<Self, LibraryFileError> }
```

## Visibility

- `pub`

## Docstring

Parse a `.snxlib` TOML document. Validates the format token, then
parses each `[tables.<name>]` block's embedded TSV into rows.

## Source
Lines 8–72 in `crates/oxide-library/src/library_file/codec.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codec](/crates/oxide-library/src/library_file/codec.md) |
| calls | [parse_tsv](/crates/oxide-library/src/library_file/codec/parse_tsv.md) |
