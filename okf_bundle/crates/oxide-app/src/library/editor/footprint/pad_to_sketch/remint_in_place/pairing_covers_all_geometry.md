---
okf_version: "0.2"
type: Function
title: pairing_covers_all_geometry
description: "True when every reference entity that CARRIES geometry — a Point's"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pairing_covers_all_geometry
language: rust
---

# pairing_covers_all_geometry

True when every reference entity that CARRIES geometry — a Point's

## Signature

```rust
fn pairing_covers_all_geometry(
    pairs: &[(SketchEntityId, SketchEntityId)],
    reference_sketch: &SketchData,
) -> bool
```

## Docstring

True when every reference entity that CARRIES geometry — a Point's
position, a Circle's radius — was paired. A Line holds nothing but
references to Points, so leaving one unpaired changes no geometry;
a stranded Point or Circle means a partial write-back.

## Source
Lines 196–207 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remint_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
