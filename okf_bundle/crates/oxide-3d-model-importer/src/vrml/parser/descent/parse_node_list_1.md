---
okf_version: "0.2"
type: Function
title: parse_node_list
description: Parse zero or more top-level or child nodes until we hit RBrace / EOF.
resource: crates/oxide-3d-model-importer/src/vrml/parser/descent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/vrml/parser/descent/parse_node_list_1
language: rust
---

# parse_node_list

Parse zero or more top-level or child nodes until we hit RBrace / EOF.

## Signature

```rust
pub(super) fn parse_node_list(&mut self) -> Result<Vec<Node>, ParseError>
```

## Visibility

- `pub(super)`

## Docstring

Parse zero or more top-level or child nodes until we hit RBrace / EOF.

## Source
Lines 54–67 in `crates/oxide-3d-model-importer/src/vrml/parser/descent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [descent](/crates/oxide-3d-model-importer/src/vrml/parser/descent.md) |
