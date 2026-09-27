---
okf_version: "0.2"
type: Class
title: EntityKind
description: "[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
resource: crates/oxide-sketch/src/entity.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/entity/EntityKind
language: rust
---

# EntityKind

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Signature

```rust
pub enum EntityKind
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `x`
- `y`
- `start`
- `end`
- `center`
- `start`
- `end`
- `sweep_ccw`
- `center`
- `radius`

## Source
Lines 60–82 in `crates/oxide-sketch/src/entity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [entity](/crates/oxide-sketch/src/entity.md) |
