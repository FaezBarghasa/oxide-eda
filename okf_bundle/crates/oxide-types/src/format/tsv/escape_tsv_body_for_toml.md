---
okf_version: "0.2"
type: Function
title: escape_tsv_body_for_toml
description: Escape a rendered TSV block so it can be embedded inside a TOML
resource: crates/oxide-types/src/format/tsv.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-types/src/format/tsv/escape_tsv_body_for_toml
language: rust
---

# escape_tsv_body_for_toml

Escape a rendered TSV block so it can be embedded inside a TOML

## Signature

```rust
pub(in crate::format) fn escape_tsv_body_for_toml(body: &str) -> String
```

## Visibility

- `pub(in crate::format)`

## Docstring

Escape a rendered TSV block so it can be embedded inside a TOML
multi-line *basic* string (`"""..."""`). TOML treats `\` as an
escape introducer and ends the string on `"""`, so a cell holding
a Windows path (`C:\...`), a literal quote, or an inch value like
`1/4"` would otherwise produce a file that can never be reopened.

Backslashes are doubled and any run of three or more quotes is
broken with `\"` (still a literal quote to TOML) so it cannot be
read as the closing delimiter. Interior newlines and tabs are
valid literally in a multi-line basic string and are preserved so
the block stays line-diffable. Every other C0 control byte (and
DEL) is illegal unescaped in a TOML basic string, so it is written
as `\uXXXX` — otherwise a cell holding e.g. a BEL or VT byte saves
fine but `toml::from_str` rejects the file on reopen (#386).

## Source
Lines 107–133 in `crates/oxide-types/src/format/tsv.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tsv](/crates/oxide-types/src/format/tsv.md) |
| called_by | [escape_tsv_body_for_toml_never_leaves_a_triple_quote_run](/crates/oxide-types/src/format/tests/escape_tsv_body_for_toml_never_leaves_a_triple_quote_run.md) |
| called_by | [write_tsv_section](/crates/oxide-types/src/format/tsv/write_tsv_section.md) |
