---
okf_version: "0.2"
type: Function
title: build_qfn16
description: "QFN-16: 0.5 mm pitch, 4 pads per side around a 3 × 3 mm body."
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library/build_qfn16
language: rust
---

# build_qfn16

QFN-16: 0.5 mm pitch, 4 pads per side around a 3 × 3 mm body.

## Signature

```rust
fn build_qfn16() -> Footprint
```

## Docstring

QFN-16: 0.5 mm pitch, 4 pads per side around a 3 × 3 mm body.
Side rows are at ±row_offset from origin; pad copper 0.3 × 0.6 mm
(long axis radial). Pin 1 is the top-left of the west side
(CCW numbering): W4..W1, S4..S1, E1..E4, N1..N4 → labelled 1..16.

## Source
Lines 218–321 in `crates/oxide-app/examples/build_stock_library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build_stock_library](/crates/oxide-app/examples/build_stock_library.md) |
| calls | [smd_pad](/crates/oxide-app/examples/build_stock_library/smd_pad.md) |
