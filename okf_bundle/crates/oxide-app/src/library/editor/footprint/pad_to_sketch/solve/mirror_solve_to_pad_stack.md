---
okf_version: "0.2"
type: Function
title: mirror_solve_to_pad_stack
description: "v0.24 Phase 3 (Track A4) — RoundRect: re-derive the"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_pad_stack
language: rust
---

# mirror_solve_to_pad_stack

v0.24 Phase 3 (Track A4) — RoundRect: re-derive the

## Signature

```rust
pub fn mirror_solve_to_pad_stack(
    state: &mut FootprintEditorState,
    resolved: &HashMap<String, f64>,
)
```

## Visibility

- `pub`

## Docstring

v0.24 Phase 3 (Track A4) — RoundRect: re-derive the
`EditorPad.stack.corner_radius_pct` value from the live
`corner_r_<slug>` parameter in `resolved`.

## Source
Lines 31–63 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
