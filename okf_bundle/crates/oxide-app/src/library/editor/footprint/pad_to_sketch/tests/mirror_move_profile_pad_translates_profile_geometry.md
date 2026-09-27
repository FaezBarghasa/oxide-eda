---
okf_version: "0.2"
type: Function
title: mirror_move_profile_pad_translates_profile_geometry
description: "Moving a `SketchProfile` pad must carry its profile geometry along."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_translates_profile_geometry
language: rust
---

# mirror_move_profile_pad_translates_profile_geometry

Moving a `SketchProfile` pad must carry its profile geometry along.

## Signature

```rust
fn mirror_move_profile_pad_translates_profile_geometry()
```

## Decorators

- `test`

## Docstring

Moving a `SketchProfile` pad must carry its profile geometry along.

Regression for the v0.14 bug: `mirror_move_pad_in_sketch` moved only the
centre Point and the `corner_entity_ids` bbox outline. A SketchProfile pad
has `corner_entity_ids: None`, so the profile stayed at its original
coordinates — visibly, the sketch rectangle did not follow the pad. The
silent half was worse: `oxide_bake` bakes the profile as
`world_pts - pad_position`, so the copper resolved back to the ORIGINAL
location and the exported footprint had the pad in the wrong place.
[test]

## Source
Lines 249–288 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [footprint_with_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/footprint_with_profile_pad.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
