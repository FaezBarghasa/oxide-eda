---
okf_version: "0.2"
type: Function
title: to_toml_string
description: Serialise to canonical TOML+TSV. Pin lists become
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/to_toml_string_1
language: rust
---

# to_toml_string

Serialise to canonical TOML+TSV. Pin lists become

## Signature

```rust
pub fn to_toml_string(&self) -> Result<String, SymbolFileError>
```

## Visibility

- `pub`

## Docstring

Serialise to canonical TOML+TSV. Pin lists become
`pins_tsv = '''\n<header>\n<rows>\n'''` literal multi-line
strings so the bulk data is line-diffable in git output.

The written `format` token is computed fresh from content —
[`SYMBOL_FILE_FORMAT_TOKEN_V2`] iff any symbol contains a
[`SymbolGraphicKind::Polygon`] graphic, else
[`SYMBOL_FILE_FORMAT_TOKEN`] — not copied from `self.format`
(which may be stale, e.g. right after loading a v1 file that
has since gained a `Polygon` in this session, or a v2 file
whose only `Polygon` was just deleted). See
[`SYMBOL_FILE_FORMAT_TOKEN_V2`]'s doc comment for why this
stays maximally backward-compatible.

## Source
Lines 779–840 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| calls | [pins_to_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv/pins_to_tsv.md) |
