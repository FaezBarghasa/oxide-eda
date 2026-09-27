---
okf_version: "0.2"
type: Module
title: sketch_dispatch
description: Phase 5.4 + 7.3 — solve-on-edit dispatcher.
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch
language: rust
---

# sketch_dispatch

Phase 5.4 + 7.3 — solve-on-edit dispatcher.

## Docstring

Phase 5.4 + 7.3 — solve-on-edit dispatcher.

Applies a [`SketchEdit`] to the footprint's `Option<SketchData>`,
then runs the LM solver, captures DOF colouring, and (Phase 7.3)
invokes `oxide_bake` to regenerate `Footprint::pads` from the
solved sketch.

Design:
- The dispatcher is a free function so it can be unit-tested
without spinning up an iced runtime.
- The solver is always live. v0.22 stripped the previous
auto-pause hysteresis: footprint sketches stay small enough
that every solve completes well under the per-frame budget,
and a "paused" state was confusing for both users and the
downstream Signal AI agent reading solver state.
- All `SketchError` cases — including timeouts — propagate as
`SketchError::SolveFailed`. The `_with_warnings` wrappers
surface them in `state.solve_warnings`; nothing is silently
swallowed.

## Relationships

| Type | Target |
|------|--------|
| related | [apply_sketch_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit.md) |
| related | [apply_sketch_edit_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_edit_with_warnings.md) |
| related | [apply_sketch_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role.md) |
| related | [apply_sketch_role_with_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_sketch_role_with_warnings.md) |
| related | [set_entity_role](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/set_entity_role.md) |
| related | [current_role_of](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/current_role_of.md) |
| related | [apply_edit_inner](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/apply_edit_inner.md) |
| related | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
