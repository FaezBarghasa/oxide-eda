---
okf_version: "0.2"
type: Function
title: parse_list
description: "`list ::= expr (',' expr)*` — parses until the next `]`."
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/parse_list_1
language: rust
---

# parse_list

`list ::= expr (',' expr)*` — parses until the next `]`.

## Signature

```rust
fn parse_list(&mut self) -> Result<Vec<ExprNode>, ExprError>
```

## Docstring

`list ::= expr (',' expr)*` — parses until the next `]`.
The list is at least one element; an empty `[]` is a parse
error.

## Source
Lines 596–604 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
