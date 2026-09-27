---
okf_version: "0.2"
type: Function
title: footprint_sketch_is_active
description: v0.15 — gate the Pads → Sketch mirror on whether the footprint already
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/footprint_sketch_is_active
language: rust
---

# footprint_sketch_is_active

v0.15 — gate the Pads → Sketch mirror on whether the footprint already

## Signature

```rust
fn footprint_sketch_is_active(fp: &oxide_library::primitive::footprint::Footprint) -> bool
```

## Docstring

v0.15 — gate the Pads → Sketch mirror on whether the footprint already
has a sketch (i.e. the user has visited Sketch mode at least once OR
auto-mint has already fired). Mirroring into a non-existent sketch would
create one silently, which is undesirable for users who only ever work
in Pads mode.

## Source
Lines 324–329 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| called_by | [add_hole](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_hole.md) |
| called_by | [add_pad](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_pad.md) |
| called_by | [add_via](/crates/oxide-app/src/library/editor/footprint/updates/geometry/add_via.md) |
