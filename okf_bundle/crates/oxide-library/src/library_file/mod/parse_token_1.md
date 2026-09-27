---
okf_version: "0.2"
type: Function
title: parse_token
description: Parse a TOML wire token back to the typed enum. Errors on
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/parse_token_1
language: rust
---

# parse_token

Parse a TOML wire token back to the typed enum. Errors on

## Signature

```rust
pub fn parse_token(s: &str) -> Result<Self, ColumnTypeParseError>
```

## Visibility

- `pub`

## Docstring

Parse a TOML wire token back to the typed enum. Errors on
unknown type names or malformed `enum:` payloads.

## Source
Lines 184–205 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
