---
okf_version: "0.2"
type: Function
title: from_toml_str
description: Parse the TOML+TSV wire format. The format-token check pins us
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/from_toml_str_1
language: rust
---

# from_toml_str

Parse the TOML+TSV wire format. The format-token check pins us

## Signature

```rust
pub fn from_toml_str(text: &str) -> Result<Self, FootprintFileError>
```

## Visibility

- `pub`

## Docstring

Parse the TOML+TSV wire format. The format-token check pins us
to [`FOOTPRINT_FILE_FORMAT_TOKEN`]; mismatched files surface
[`FootprintFileError::UnsupportedFormat`].

## Source
Lines 626–674 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
| calls | [pads_from_tsv](/crates/oxide-library/src/primitive/footprint/serde_tsv/pads_from_tsv.md) |
