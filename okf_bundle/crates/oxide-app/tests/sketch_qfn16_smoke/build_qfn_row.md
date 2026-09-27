---
okf_version: "0.2"
type: Function
title: build_qfn_row
description: "Author one row of QFN pads, parameterised on `pad_pitch`. Pads"
resource: crates/oxide-app/tests/sketch_qfn16_smoke.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/tests/sketch_qfn16_smoke/build_qfn_row
language: rust
---

# build_qfn_row

Author one row of QFN pads, parameterised on `pad_pitch`. Pads

## Signature

```rust
fn build_qfn_row(pad_pitch_expr: &str) -> (Footprint, Vec<SketchEntityId>)
```

## Docstring

Author one row of QFN pads, parameterised on `pad_pitch`. Pads
are at `(ROW_X, +1.5 * pad_pitch)`, `(ROW_X, +0.5 * pad_pitch)`,
`(ROW_X, -0.5 * pad_pitch)`, `(ROW_X, -1.5 * pad_pitch)`. Pitch
resolution flows via the parameter table.

## Source
Lines 37–84 in `crates/oxide-app/tests/sketch_qfn16_smoke.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sketch_qfn16_smoke](/crates/oxide-app/tests/sketch_qfn16_smoke.md) |
| called_by | [qfn16_row_bakes_at_05mm_pitch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_bakes_at_05mm_pitch.md) |
| called_by | [qfn16_row_regenerates_when_pad_pitch_changes](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_regenerates_when_pad_pitch_changes.md) |
| called_by | [qfn16_solve_warnings_empty_on_clean_sketch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_solve_warnings_empty_on_clean_sketch.md) |
