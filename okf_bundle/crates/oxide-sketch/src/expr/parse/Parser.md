---
okf_version: "0.2"
type: Class
title: Parser
description: Recursive-descent parser. Holds a single token of look-ahead so
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/Parser
language: rust
---

# Parser

Recursive-descent parser. Holds a single token of look-ahead so

## Signature

```rust
struct Parser
```

## Type Parameters

- `'a`

## Docstring

Recursive-descent parser. Holds a single token of look-ahead so
each grammar method can decide its production by inspecting
`self.peek`.

## Methods

- `lexer`
- `peek`

## Source
Lines 338–342 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
