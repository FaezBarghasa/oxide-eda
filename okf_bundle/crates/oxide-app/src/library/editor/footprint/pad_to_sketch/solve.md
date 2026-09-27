---
okf_version: "0.2"
type: Module
title: solve
description: "Post-solve \"reverse mirror\" helpers — when a solver run rewrites"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve
language: rust
---

# solve

Post-solve "reverse mirror" helpers — when a solver run rewrites

## Docstring

Post-solve "reverse mirror" helpers — when a solver run rewrites
a sketch parameter (e.g. via the parameter table), these helpers
propagate the resolved value back into the literal pad-stack
geometry so the canvas + Pads-mode Properties stay in sync.

All helpers follow the same defensive pattern: skip pads whose
shape doesn't carry the relevant binding, and silently early-out
when the bound parameter isn't present in `resolved`.

## Relationships

| Type | Target |
|------|--------|
| related | [mirror_solve_to_pad_stack](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_pad_stack.md) |
| related | [mirror_solve_to_oval_size](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_size.md) |
| related | [mirror_solve_to_chamfer_anchors](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_chamfer_anchors.md) |
| related | [mirror_solve_to_round_rect_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_round_rect_geometry.md) |
| related | [mirror_solve_to_oval_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/mirror_solve_to_oval_geometry.md) |
| related | [sidecar_to_id](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/sidecar_to_id.md) |
| related | [move_anchor_via_sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/solve/move_anchor_via_sidecar.md) |
