---
okf_version: "0.2"
type: Function
title: convex_filled_pad
description: "Convex quad standing in for a filled SMD pad: fill only, no stroke."
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/convex_filled_pad
language: rust
---

# convex_filled_pad

Convex quad standing in for a filled SMD pad: fill only, no stroke.

## Signature

```rust
fn convex_filled_pad() -> GpuPolygon
```

## Docstring

Convex quad standing in for a filled SMD pad: fill only, no stroke.

## Source
Lines 63–70 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| called_by | [triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_convex_pad_fans_exactly_n_minus_2_fill_triangles.md) |
