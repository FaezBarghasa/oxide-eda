---
okf_version: "0.2"
type: Module
title: sketch_qfn16_smoke
description: Phase 8.1 — QFN-16 end-to-end author smoke.
resource: crates/oxide-app/tests/sketch_qfn16_smoke.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/tests/sketch_qfn16_smoke
language: rust
---

# sketch_qfn16_smoke

Phase 8.1 — QFN-16 end-to-end author smoke.

## Docstring

Phase 8.1 — QFN-16 end-to-end author smoke.

Programmatically builds a SketchData representing one row of a
QFN-16 footprint (4 pads on the east side, 0.5 mm pitch), runs the
solve-on-edit dispatcher, asserts the baked Pad coordinates match
to within 1 µm, then mutates `pad_pitch` to 0.65 mm and re-asserts
the regenerated coordinates.

Drives the entire v0.13 stack: parameter resolution → expression
evaluation → LM solver → DOF analysis → pad bake → sketch ↔ library
integration. No UI required.

## Relationships

| Type | Target |
|------|--------|
| related | [build_qfn_row](/crates/oxide-app/tests/sketch_qfn16_smoke/build_qfn_row.md) |
| related | [qfn16_row_bakes_at_05mm_pitch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_bakes_at_05mm_pitch.md) |
| related | [qfn16_row_regenerates_when_pad_pitch_changes](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_row_regenerates_when_pad_pitch_changes.md) |
| related | [qfn16_solve_warnings_empty_on_clean_sketch](/crates/oxide-app/tests/sketch_qfn16_smoke/qfn16_solve_warnings_empty_on_clean_sketch.md) |
| related | [_link_chrono](/crates/oxide-app/tests/sketch_qfn16_smoke/link_chrono.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
