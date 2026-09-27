---
okf_version: "0.2"
type: Function
title: reverse_mirror_updates_pad_stack_corner_radius_pct
description: "v0.24 Phase 3 (Track A4) — after every solve, the reverse mirror"
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/reverse_mirror_updates_pad_stack_corner_radius_pct
language: rust
---

# reverse_mirror_updates_pad_stack_corner_radius_pct

v0.24 Phase 3 (Track A4) — after every solve, the reverse mirror

## Signature

```rust
fn reverse_mirror_updates_pad_stack_corner_radius_pct()
```

## Decorators

- `test`

## Docstring

v0.24 Phase 3 (Track A4) — after every solve, the reverse mirror
re-derives `EditorPad.stack.corner_radius_pct` from the resolved
corner_r parameter so the Pads-mode "Corner radius %" input stays
in sync with sketch-side edits.
[test]

## Source
Lines 495–564 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
