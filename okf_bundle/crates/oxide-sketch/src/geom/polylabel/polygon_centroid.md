---
okf_version: "0.2"
type: Function
title: polygon_centroid
description: Centroid of a polygon (average of vertex coordinates). Not the
resource: crates/oxide-sketch/src/geom/polylabel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/polylabel/polygon_centroid
language: rust
---

# polygon_centroid

Centroid of a polygon (average of vertex coordinates). Not the

## Signature

```rust
fn polygon_centroid(polygon: &[Point2]) -> Point2
```

## Docstring

Centroid of a polygon (average of vertex coordinates). Not the
same as the centroid-of-area but adequate as a starting seed
for the polylabel search.

## Source
Lines 164–169 in `crates/oxide-sketch/src/geom/polylabel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polylabel](/crates/oxide-sketch/src/geom/polylabel.md) |
| called_by | [pole_of_inaccessibility](/crates/oxide-sketch/src/geom/polylabel/pole_of_inaccessibility.md) |
