---
okf_version: "0.2"
type: Function
title: from_toml_str
description: Parse the TOML+TSV wire format. The format-token check accepts
resource: crates/oxide-library/src/primitive/symbol/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/mod/from_toml_str_1
language: rust
---

# from_toml_str

Parse the TOML+TSV wire format. The format-token check accepts

## Signature

```rust
pub fn from_toml_str(text: &str) -> Result<Self, SymbolFileError>
```

## Visibility

- `pub`

## Docstring

Parse the TOML+TSV wire format. The format-token check accepts
either [`SYMBOL_FILE_FORMAT_TOKEN`] or
[`SYMBOL_FILE_FORMAT_TOKEN_V2`]; any other token surfaces
[`SymbolFileError::UnsupportedFormat`].

Every loaded `Arc` graphic passes through
[`migrate_legacy_arc`], which self-heals two pre-normalization
authoring bugs in stored `start_deg`/`end_deg` pairs — see that
function's doc comment. This runs on load (not just for v1
files) so every consumer of a `Symbol` gets already-migrated
data regardless of entry point, and the next save re-emits the
corrected values, healing the file on disk too.

## Source
Lines 713–764 in `crates/oxide-library/src/primitive/symbol/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-library/src/primitive/symbol/mod.md) |
| calls | [pins_from_tsv](/crates/oxide-library/src/primitive/symbol/serde_tsv/pins_from_tsv.md) |
| calls | [migrate_legacy_arc](/crates/oxide-library/src/primitive/symbol/mod/migrate_legacy_arc.md) |
