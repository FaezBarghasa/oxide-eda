---
okf_version: "0.2"
type: Function
title: parse_node
description: "Parse a single VRML node, returning `None` for ignored node types."
resource: crates/oxide-3d-model-importer/src/vrml/parser/descent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/descent/parse_node
language: rust
---

# parse_node

Parse a single VRML node, returning `None` for ignored node types.

## Signature

```rust
impl Parser<'a> { fn parse_node(&mut self) -> Result<Option<Node>, ParseError> }
```

## Type Parameters

- `'a`

## Docstring

Parse a single VRML node, returning `None` for ignored node types.

## Source
Lines 70–101 in `crates/oxide-3d-model-importer/src/vrml/parser/descent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [descent](/crates/oxide-3d-model-importer/src/vrml/parser/descent.md) |
