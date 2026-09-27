---
okf_version: "0.2"
type: Function
title: placement_input_circle_radius_pins_typed_radius
description: "v0.14-footprint re-verify — Circle still honours a typed radius:"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_circle_radius_pins_typed_radius
language: rust
---

# placement_input_circle_radius_pins_typed_radius

v0.14-footprint re-verify — Circle still honours a typed radius:

## Signature

```rust
fn placement_input_circle_radius_pins_typed_radius()
```

## Decorators

- `test`

## Docstring

v0.14-footprint re-verify — Circle still honours a typed radius:
click the centre, type "4", then a click at the cursor's 10 mm
must commit a circle of radius 4 (the typed value), not 10.
[test]

## Source
Lines 1099–1160 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
