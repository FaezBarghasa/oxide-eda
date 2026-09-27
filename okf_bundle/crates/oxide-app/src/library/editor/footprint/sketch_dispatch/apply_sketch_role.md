---
okf_version: "0.2"
type: Function
title: apply_sketch_role
description: "v0.16.2 — apply a [`RoleTag`] change to the entity at `id`."
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role
language: rust
---

# apply_sketch_role

v0.16.2 — apply a [`RoleTag`] change to the entity at `id`.

## Signature

```rust
pub fn apply_sketch_role(
    state: &mut FootprintEditorState,
    footprint: &mut Footprint,
    id: SketchEntityId,
    role: RoleTag,
) -> Result<(), SketchError>
```

## Visibility

- `pub`

## Docstring

v0.16.2 — apply a [`RoleTag`] change to the entity at `id`.

Behaviour:
1. Snapshot the next pad-designator from the existing pad attrs
(excluding the target entity) so re-assigning Pad after a clear
doesn't double-issue a number.
2. Clear every `*Attr` slot on the target entity.
3. Set the matching attr per `role` with sensible defaults. Pad
role on a non-Point entity is a silent no-op (the construction
geometry doesn't carry a position the bake can use).
4. Run a solve + bake so the new geometry materialises in
`Footprint::pads / silk_f / silk_b / courtyard / mask_openings
/ mask_excludes / paste_apertures / pours / keepouts / cutouts`.

Returns `Err(SketchError)` only if the solver fails. The
no-op-on-non-Point case returns `Ok(())` so the caller can shrug
it off without inspecting the geometry.

## Source
Lines 76–84 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch.md) |
| calls | [set_entity_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/set_entity_role.md) |
| calls | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
| called_by | [apply_sketch_role_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role_with_warnings.md) |
| called_by | [set_role_courtyard_attaches_courtyard_attr](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_courtyard_attaches_courtyard_attr.md) |
| called_by | [set_role_pad_increments_designator_across_entities](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_increments_designator_across_entities.md) |
| called_by | [set_role_pad_on_line_is_silent_noop](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_on_line_is_silent_noop.md) |
| called_by | [set_role_pad_on_point_attaches_pad_attr_and_bakes](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_pad_on_point_attaches_pad_attr_and_bakes.md) |
| called_by | [set_role_replaces_existing_attr_atomically](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_replaces_existing_attr_atomically.md) |
| called_by | [set_role_silk_top_attaches_silk_attr_with_top_layer](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_silk_top_attaches_silk_attr_with_top_layer.md) |
| called_by | [set_role_unassigned_clears_every_attr](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_unassigned_clears_every_attr.md) |
