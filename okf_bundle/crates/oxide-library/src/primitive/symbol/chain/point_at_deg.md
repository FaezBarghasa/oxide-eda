---
okf_version: "0.2"
type: Function
title: point_at_deg
description: "Point at angle `deg` (degrees) on the circle — matches"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain/point_at_deg
language: rust
---

# point_at_deg

Point at angle `deg` (degrees) on the circle — matches

## Signature

```rust
fn point_at_deg(center: [f64; 2], radius: f64, deg: f64) -> [f64; 2]
```

## Docstring

Point at angle `deg` (degrees) on the circle — matches
`SymbolGraphicKind::Arc`'s endpoint-handle math in `hit_test.rs`.

## Source
Lines 492–498 in `crates/oxide-library/src/primitive/symbol/chain.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chain](/crates/oxide-library/src/primitive/symbol/chain.md) |
| called_by | [tessellate_segment](/crates/oxide-library/src/primitive/symbol/chain/tessellate_segment.md) |
