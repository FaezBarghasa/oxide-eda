---
okf_version: "0.2"
type: Function
title: pin_orientation_maps_to_rotation_degrees_for_all_four_variants
description: "Acceptance criterion: pin the complete `PinOrientation` -> rotation"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests/pin_orientation_maps_to_rotation_degrees_for_all_four_variants
language: rust
---

# pin_orientation_maps_to_rotation_degrees_for_all_four_variants

Acceptance criterion: pin the complete `PinOrientation` -> rotation

## Signature

```rust
fn pin_orientation_maps_to_rotation_degrees_for_all_four_variants()
```

## Decorators

- `test`

## Docstring

Acceptance criterion: pin the complete `PinOrientation` -> rotation
table (all four variants). The mapping is the identity on angle —
`LibPin.pin.rotation` is read as the same y-up, CCW-from-+x,
tip->body angle as the source `PinOrientation` by every real
consumer: `oxide_types::schematic::SymbolTransform::apply` (the
single y-flip is applied there, not here), the autoplace pin-bbox
walk (`crates/oxide-engine/src/transform/autoplace.rs`), and the
SVG/PDF exporter's `pin_direction`
(`crates/oxide-output/src/svg/symbols.rs`, `90 => (0.0, 1.0)`). See
`to_lib_symbol`'s module doc for the full derivation.
[test]

## Source
Lines 119–135 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol_tests](/crates/oxide-library/src/primitive/symbol/to_lib_symbol_tests.md) |
