---
okf_version: "0.2"
type: Function
title: mm_distance
description: "Euclidean distance between two `(x, y)` pairs in mm."
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/mm_distance
language: rust
---

# mm_distance

Euclidean distance between two `(x, y)` pairs in mm.

## Signature

```rust
fn mm_distance(a: (f64, f64), b: (f64, f64) -> f64
```

## Docstring

Euclidean distance between two `(x, y)` pairs in mm.

## Source
Lines 325–327 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| called_by | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
| called_by | [validate_split](/crates/oxide-sketch/src/split/mod/validate_split.md) |
