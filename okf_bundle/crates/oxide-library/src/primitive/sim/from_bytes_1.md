---
okf_version: "0.2"
type: Function
title: from_bytes
description: "Decode bytes as UTF-8 and parse via [`SimFile::from_toml_str`]."
resource: crates/oxide-library/src/primitive/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:11:54Z"
concept_id: crates/oxide-library/src/primitive/sim/from_bytes_1
language: rust
---

# from_bytes

Decode bytes as UTF-8 and parse via [`SimFile::from_toml_str`].

## Signature

```rust
pub fn from_bytes(bytes: &[u8]) -> Result<Self, SimFileError>
```

## Visibility

- `pub`

## Docstring

Decode bytes as UTF-8 and parse via [`SimFile::from_toml_str`].

## Source
Lines 156–162 in `crates/oxide-library/src/primitive/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-library/src/primitive/sim.md) |
