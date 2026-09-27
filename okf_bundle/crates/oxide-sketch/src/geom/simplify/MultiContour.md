---
okf_version: "0.2"
type: Class
title: MultiContour
description: "Multi-contour polygon: one outer ring plus zero or more holes."
resource: crates/oxide-sketch/src/geom/simplify.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/simplify/MultiContour
language: rust
---

# MultiContour

Multi-contour polygon: one outer ring plus zero or more holes.

## Signature

```rust
pub struct MultiContour
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq)`

## Visibility

- `pub`

## Docstring

Multi-contour polygon: one outer ring plus zero or more holes.
Outer winds CCW; holes wind CW by convention. Useful as a
type to express "polygon with holes" even before the full
hole-aware boolean implementation lands.
[derive(Debug, Clone, Default, PartialEq)]

## Methods

- `outer`
- `holes`

## Source
Lines 21–24 in `crates/oxide-sketch/src/geom/simplify.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [simplify](/crates/oxide-sketch/src/geom/simplify.md) |
