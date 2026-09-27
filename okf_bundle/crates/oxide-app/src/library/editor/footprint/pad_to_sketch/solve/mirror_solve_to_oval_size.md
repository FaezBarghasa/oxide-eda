---
okf_version: "0.2"
type: Function
title: mirror_solve_to_oval_size
description: "v0.25 polish — Oval reverse-mirror: when the user edits"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_size
language: rust
---

# mirror_solve_to_oval_size

v0.25 polish — Oval reverse-mirror: when the user edits

## Signature

```rust
pub fn mirror_solve_to_oval_size(
    state: &mut FootprintEditorState,
    resolved: &HashMap<String, f64>,
)
```

## Visibility

- `pub`

## Docstring

v0.25 polish — Oval reverse-mirror: when the user edits
`width_<slug>` or `height_<slug>` from the Properties panel, the
resolved value should also propagate back to `pad.size_mm`.

## Source
Lines 68–108 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solve](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.md) |
| called_by | [solve_and_bake](/crates/oxide-app/src/library/editor/footprint/sketch_dispatch/solve_and_bake.md) |
