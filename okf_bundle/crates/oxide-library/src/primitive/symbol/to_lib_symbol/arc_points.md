---
okf_version: "0.2"
type: Function
title: arc_points
description: "`SymbolGraphicKind::Arc`'s `start_deg..end_deg` always sweeps"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/arc_points
language: rust
---

# arc_points

`SymbolGraphicKind::Arc`'s `start_deg..end_deg` always sweeps

## Signature

```rust
fn arc_points(
    center: [f64; 2],
    radius: f64,
    start_deg: f64,
    end_deg: f64,
) -> (Point, Point, Point)
```

## Docstring

`SymbolGraphicKind::Arc`'s `start_deg..end_deg` always sweeps
counter-clockwise, wrapping a full turn when `end_deg < start_deg`
(the same convention `chain.rs` and `normalize_arc_endpoints_deg`
document/enforce). `Graphic::Arc` has no angle fields at all — just
three Cartesian points — so the midpoint angle along that same CCW
sweep is what stands in for the missing "which way does it bulge"
information.

## Source
Lines 265–278 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| calls | [point_at_deg](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/point_at_deg.md) |
| called_by | [to_graphic](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/to_graphic.md) |
