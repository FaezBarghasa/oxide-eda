---
okf_version: "0.2"
type: Function
title: copy_entity_geometry
description: "Copy one reference entity's geometry onto the entity it pairs with."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/copy_entity_geometry
language: rust
---

# copy_entity_geometry

Copy one reference entity's geometry onto the entity it pairs with.

## Signature

```rust
fn copy_entity_geometry(
    from: &SketchData,
    from_id: SketchEntityId,
    into: &mut SketchData,
    into_id: SketchEntityId,
)
```

## Docstring

Copy one reference entity's geometry onto the entity it pairs with.
Points carry their position, Circles their radius, Arcs their sweep;
a Line's geometry is entirely in the Points it references. The
centre also carries its `PadAttr`, which is how the size expressions
`oxide_bake::pad` reads stay in step with the new frame.

## Source
Lines 214–239 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remint_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
