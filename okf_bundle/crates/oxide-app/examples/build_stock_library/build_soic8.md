---
okf_version: "0.2"
type: Function
title: build_soic8
description: "JEDEC SOIC-8: 1.27 mm pitch, body ≈ 3.9 × 4.9 mm, two rows of 4"
resource: crates/oxide-app/examples/build_stock_library.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/examples/build_stock_library/build_soic8
language: rust
---

# build_soic8

JEDEC SOIC-8: 1.27 mm pitch, body ≈ 3.9 × 4.9 mm, two rows of 4

## Signature

```rust
fn build_soic8() -> Footprint
```

## Docstring

JEDEC SOIC-8: 1.27 mm pitch, body ≈ 3.9 × 4.9 mm, two rows of 4
pads with 5.4 mm row-to-row centre spacing (IPC-7351 nominal).
Pad copper 0.6 × 1.55 mm, mask margin 0.05 mm.

## Source
Lines 145–210 in `crates/oxide-app/examples/build_stock_library.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [build_stock_library](/crates/oxide-app/examples/build_stock_library.md) |
| calls | [smd_pad](/crates/oxide-app/examples/build_stock_library/smd_pad.md) |
