---
okf_version: "0.2"
type: Function
title: from_bytes
description: "Decode bytes as UTF-8 and parse via [`FootprintFile::from_toml_str`]."
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/from_bytes_1
language: rust
---

# from_bytes

Decode bytes as UTF-8 and parse via [`FootprintFile::from_toml_str`].

## Signature

```rust
pub fn from_bytes(bytes: &[u8]) -> Result<Self, FootprintFileError>
```

## Visibility

- `pub`

## Docstring

Decode bytes as UTF-8 and parse via [`FootprintFile::from_toml_str`].

## Source
Lines 615–621 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
