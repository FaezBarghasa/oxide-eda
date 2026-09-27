---
okf_version: "0.2"
type: Class
title: Segment2
description: "2D line segment from `a` to `b`. Endpoints stored as bare points"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/Segment2
language: rust
---

# Segment2

2D line segment from `a` to `b`. Endpoints stored as bare points

## Signature

```rust
pub struct Segment2
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

2D line segment from `a` to `b`. Endpoints stored as bare points
so the user can build one inline from `EntityKind::Point` data.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `a`
- `b`

## Source
Lines 14–17 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
