---
okf_version: "0.2"
type: Class
title: Point2
description: "Plain-old 2D point in plane-local mm. The crate's `EntityKind::Point`"
resource: crates/oxide-sketch/src/geom/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/mod/Point2
language: rust
---

# Point2

Plain-old 2D point in plane-local mm. The crate's `EntityKind::Point`

## Signature

```rust
pub struct Point2
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

Plain-old 2D point in plane-local mm. The crate's `EntityKind::Point`
uses bare `(x, y): f64` fields; this struct lets the geom helpers
take an idiomatic struct argument while staying convertible from
tuples and arrays so existing call sites can pass either.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `x`
- `y`

## Source
Lines 54–57 in `crates/oxide-sketch/src/geom/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geom](/crates/oxide-sketch/src/geom/mod.md) |
