---
okf_version: "0.2"
type: Function
title: editing_corner_radius_updates_all_4_arcs
description: v0.24 Phase 3 (Track A2) — dispatching FpEditorEditPadShapeParam
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/editing_corner_radius_updates_all_4_arcs
language: rust
---

# editing_corner_radius_updates_all_4_arcs

v0.24 Phase 3 (Track A2) — dispatching FpEditorEditPadShapeParam

## Signature

```rust
fn editing_corner_radius_updates_all_4_arcs()
```

## Decorators

- `test`

## Docstring

v0.24 Phase 3 (Track A2) — dispatching FpEditorEditPadShapeParam
rewrites the bound sketch parameter and triggers a solve+rebake
(warnings list stays empty).
[test]

## Source
Lines 313–383 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
