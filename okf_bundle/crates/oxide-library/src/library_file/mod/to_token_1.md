---
okf_version: "0.2"
type: Function
title: to_token
description: Encode the type as the string token written in TOML. Unit
resource: crates/oxide-library/src/library_file/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/library_file/mod/to_token_1
language: rust
---

# to_token

Encode the type as the string token written in TOML. Unit

## Signature

```rust
pub fn to_token(&self) -> String
```

## Visibility

- `pub`

## Docstring

Encode the type as the string token written in TOML. Unit
variants emit the bare type name; [`ColumnType::Enum`] emits
`"enum:value1,value2,..."` so the wire format is one cell per
row and human-readable in `git diff`.

## Source
Lines 166–180 in `crates/oxide-library/src/library_file/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_file](/crates/oxide-library/src/library_file/mod.md) |
