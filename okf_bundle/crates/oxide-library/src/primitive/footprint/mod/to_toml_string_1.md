---
okf_version: "0.2"
type: Function
title: to_toml_string
description: Serialise to canonical TOML+TSV. Pad lists become
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/to_toml_string_1
language: rust
---

# to_toml_string

Serialise to canonical TOML+TSV. Pad lists become

## Signature

```rust
pub fn to_toml_string(&self) -> Result<String, FootprintFileError>
```

## Visibility

- `pub`

## Docstring

Serialise to canonical TOML+TSV. Pad lists become
`pads_tsv = '''\n<header>\n<rows>\n'''` literal multi-line
strings so the bulk data is line-diffable in git output.

## Source
Lines 679–731 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
| calls | [pads_to_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv/pads_to_tsv.md) |
