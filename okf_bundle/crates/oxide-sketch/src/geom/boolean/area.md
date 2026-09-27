---
okf_version: "0.2"
type: Function
title: area
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean/area
language: rust
---

# area

## Signature

```rust
fn area(poly: &[Point2]) -> f64
```

## Source
Lines 137–139 in `crates/oxide-sketch/src/geom/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-sketch/src/geom/boolean.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
