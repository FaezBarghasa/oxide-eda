---
okf_version: "0.2"
type: Class
title: Spanned
description: A token paired with the byte offset of its first character in the
resource: crates/oxide-sketch/src/expr/parse.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/expr/parse/Spanned
language: rust
---

# Spanned

A token paired with the byte offset of its first character in the

## Signature

```rust
struct Spanned
```

## Decorators

- `derive(Clone, Debug)`

## Docstring

A token paired with the byte offset of its first character in the
source string. The position is used solely for error messages.
[derive(Clone, Debug)]

## Methods

- `tok`
- `pos`

## Source
Lines 88–91 in `crates/oxide-sketch/src/expr/parse.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parse](/crates/oxide-sketch/src/expr/parse.md) |
