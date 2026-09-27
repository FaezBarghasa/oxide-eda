---
okf_version: "0.2"
type: Function
title: lex_quantity
description: "Eat a quantity literal: digits/dot followed by optional letter"
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/lex_quantity_1
language: rust
---

# lex_quantity

Eat a quantity literal: digits/dot followed by optional letter

## Signature

```rust
fn lex_quantity(&mut self) -> Token
```

## Docstring

Eat a quantity literal: digits/dot followed by optional letter
suffix. The cursor is positioned at a digit when this is called.

## Source
Lines 292–313 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
| calls | [Quantity](/crates/oxide-sketch/src/unit/Quantity.md) |
