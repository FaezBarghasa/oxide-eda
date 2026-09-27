---
okf_version: "0.2"
type: Function
title: editing_chamfer_len_propagates_through_solve
description: "v0.24 Track A6 — editing the shared `chamfer_len_<slug>`"
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/editing_chamfer_len_propagates_through_solve
language: rust
---

# editing_chamfer_len_propagates_through_solve

v0.24 Track A6 — editing the shared `chamfer_len_<slug>`

## Signature

```rust
fn editing_chamfer_len_propagates_through_solve()
```

## Decorators

- `test`

## Docstring

v0.24 Track A6 — editing the shared `chamfer_len_<slug>`
parameter routes through the FootprintSketchEditParameter
dispatch path: rewrites the sketch parameter, runs a fresh
solve+rebake, and the post-solve chamfer-anchor mirror
(`mirror_solve_to_chamfer_anchors`) rewrites the anchor Point
coordinates from the resolved chamfer_len value. Verifies that
the shared-parameter wiring is end-to-end live (parameter
rewrite → solve → entity-position update).
[test]

## Source
Lines 974–1138 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
