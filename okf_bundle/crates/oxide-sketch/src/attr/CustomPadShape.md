---
okf_version: "0.2"
type: Class
title: CustomPadShape
description: "Source of a `PadShape::Custom` polygon."
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/CustomPadShape
language: rust
---

# CustomPadShape

Source of a `PadShape::Custom` polygon.

## Signature

```rust
pub enum CustomPadShape
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`
- `serde(tag = "kind", rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

Source of a `PadShape::Custom` polygon.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
[serde(tag = "kind", rename_all = "PascalCase")]

## Methods

- `points`
- `source`

## Source
Lines 378–384 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
