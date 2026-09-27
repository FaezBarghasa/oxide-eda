---
okf_version: "0.2"
type: Function
title: distance_to
description: Minimum distance from this bounding box to another bounding box.
resource: crates/oxide-router/src/geometry/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:59:54Z"
concept_id: crates/oxide-router/src/geometry/mod/distance_to_2
language: rust
---

# distance_to

Minimum distance from this bounding box to another bounding box.

## Signature

```rust
impl BoundingBox { pub fn distance_to(&self, other: &BoundingBox) -> Microns }
```

## Visibility

- `pub`

## Docstring

Minimum distance from this bounding box to another bounding box.

## Source
Lines 133–159 in `crates/oxide-router/src/geometry/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-router/src/geometry/mod.md) |
