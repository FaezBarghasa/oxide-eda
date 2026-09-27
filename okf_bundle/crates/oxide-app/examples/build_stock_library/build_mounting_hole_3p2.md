---
okf_version: "0.2"
type: Function
title: build_mounting_hole_3p2
description: Single non-plated through-hole at the origin. 3.2 mm drill
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library/build_mounting_hole_3p2
language: rust
---

# build_mounting_hole_3p2

Single non-plated through-hole at the origin. 3.2 mm drill

## Signature

```rust
fn build_mounting_hole_3p2() -> Footprint
```

## Docstring

Single non-plated through-hole at the origin. 3.2 mm drill
(clearance for an M3 fastener). The "pad" copper is suppressed by
using a copper-free annular zone equal to the pad size: we set the
pad outer to 6.0 mm round but the kind=NptHole means the bake
emits an unplated hole. Side=All (drill cuts both copper sides).

## Source
Lines 374–417 in `crates/oxide-app/examples/build_stock_library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build_stock_library](/crates/oxide-app/examples/build_stock_library.md) |
