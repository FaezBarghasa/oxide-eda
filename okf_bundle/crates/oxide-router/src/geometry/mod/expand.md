---
okf_version: "0.2"
type: Function
title: expand
description: Expand bounding box by margin in all directions.
resource: crates/oxide-router/src/geometry/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:59:54Z"
concept_id: crates/oxide-router/src/geometry/mod/expand
language: rust
---

# expand

Expand bounding box by margin in all directions.

## Signature

```rust
impl BoundingBox { pub fn expand(&self, margin: Microns) -> Self }
```

## Visibility

- `pub`

## Docstring

Expand bounding box by margin in all directions.

## Source
Lines 162–167 in `crates/oxide-router/src/geometry/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-router/src/geometry/mod.md) |
