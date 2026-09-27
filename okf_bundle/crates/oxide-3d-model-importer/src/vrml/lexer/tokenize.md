---
okf_version: "0.2"
type: Function
title: tokenize
description: Tokenize a VRML97 file into a flat token stream.
resource: crates/oxide-3d-model-importer/src/vrml/lexer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/lexer/tokenize
language: rust
---

# tokenize

Tokenize a VRML97 file into a flat token stream.

## Signature

```rust
pub fn tokenize(source: &str) -> (Vec<Token>, Vec<usize>)
```

## Visibility

- `pub`

## Docstring

Tokenize a VRML97 file into a flat token stream.
Comments (`# ... \n`) are stripped. Returns (tokens, line_offsets)
where `line_offsets[i]` is the 1-based source line for token `i`.

## Source
Lines 15–88 in `crates/oxide-3d-model-importer/src/vrml/lexer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lexer](/crates/oxide-3d-model-importer/src/vrml/lexer.md) |
| called_by | [tokenize_braces_and_brackets](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize_braces_and_brackets.md) |
| called_by | [tokenize_empty_is_empty](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize_empty_is_empty.md) |
| called_by | [tokenize_strips_comment](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize_strips_comment.md) |
| called_by | [tokenize_words_across_lines](/crates/oxide-3d-model-importer/src/vrml/lexer/tokenize_words_across_lines.md) |
| called_by | [load](/crates/oxide-3d-model-importer/src/vrml/mod/load.md) |
| called_by | [parse_src](/crates/oxide-3d-model-importer/src/vrml/parser/mod/parse_src.md) |
