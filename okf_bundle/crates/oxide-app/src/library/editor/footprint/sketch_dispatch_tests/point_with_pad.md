---
okf_version: "0.2"
type: Function
title: point_with_pad
resource: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/point_with_pad
language: rust
---

# point_with_pad

## Signature

```rust
fn point_with_pad(plane: PlaneId, x: f64, y: f64, number: &str) -> (Entity, SketchEntityId)
```

## Source
Lines 25–45 in `crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_dispatch_tests](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests.md) |
| called_by | [a_failed_solve_clears_the_previous_solves_colours](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/a_failed_solve_clears_the_previous_solves_colours.md) |
| called_by | [add_constraint_solves_geometry](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_constraint_solves_geometry.md) |
| called_by | [add_entity_triggers_solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/add_entity_triggers_solve_and_bake.md) |
| called_by | [set_role_replaces_existing_attr_atomically](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_replaces_existing_attr_atomically.md) |
| called_by | [set_role_unassigned_clears_every_attr](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/set_role_unassigned_clears_every_attr.md) |
| called_by | [solver_errors_surface_in_solve_warnings_not_silently_swallowed](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_errors_surface_in_solve_warnings_not_silently_swallowed.md) |
| called_by | [solver_runs_on_every_edit](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/solver_runs_on_every_edit.md) |
| called_by | [warning_wrapper_captures_parse_error_into_solve_warnings](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch_tests/warning_wrapper_captures_parse_error_into_solve_warnings.md) |
