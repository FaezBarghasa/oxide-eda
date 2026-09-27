---
okf_version: "0.2"
type: Class
title: Unit
description: "A unit attached to a [`Quantity`]."
resource: crates/oxide-sketch/src/unit.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/unit/Unit
language: rust
---

# Unit

A unit attached to a [`Quantity`].

## Signature

```rust
pub enum Unit
```

## Decorators

- `derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A unit attached to a [`Quantity`].

Units are partitioned into [`UnitFamily`]s; conversions are
only legal within a family.
[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Source
Lines 20–35 in `crates/oxide-sketch/src/unit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [unit](/crates/oxide-sketch/src/unit.md) |
