---
okf_version: "0.2"
type: Function
title: from_bytes
description: "Decode bytes as UTF-8 and parse via [`SymbolFile::from_toml_str`]."
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/from_bytes
language: rust
---

# from_bytes

Decode bytes as UTF-8 and parse via [`SymbolFile::from_toml_str`].

## Signature

```rust
impl SymbolFile { pub fn from_bytes(bytes: &[u8]) -> Result<Self, SymbolFileError> }
```

## Visibility

- `pub`

## Docstring

Decode bytes as UTF-8 and parse via [`SymbolFile::from_toml_str`].

## Source
Lines 693–699 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
