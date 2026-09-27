---
okf_version: "0.2"
type: Class
title: NumberingScheme
description: "Pad-numbering scheme for an array's expanded primitives."
resource: crates/oxide-sketch/src/array.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/array/NumberingScheme
language: rust
---

# NumberingScheme

Pad-numbering scheme for an array's expanded primitives.

## Signature

```rust
pub enum NumberingScheme
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

Pad-numbering scheme for an array's expanded primitives.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `start_expr`
- `step_expr`
- `skip_letters`
- `start_row`
- `start_col`
- `names`

## Source
Lines 96–116 in `crates/oxide-sketch/src/array.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [array](/crates/oxide-sketch/src/array.md) |
