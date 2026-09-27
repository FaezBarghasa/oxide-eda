---
okf_version: "0.2"
type: Function
title: unlink_one_corner_only_that_arc_reads_per_corner_param
description: "Phase-5 #5 — Unlink one corner only (NE), then verify the data"
resource: crates/oxide-app/tests/regression/library_cross_track.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_cross_track/unlink_one_corner_only_that_arc_reads_per_corner_param
language: rust
---

# unlink_one_corner_only_that_arc_reads_per_corner_param

Phase-5 #5 — Unlink one corner only (NE), then verify the data

## Signature

```rust
fn unlink_one_corner_only_that_arc_reads_per_corner_param()
```

## Decorators

- `test`

## Docstring

Phase-5 #5 — Unlink one corner only (NE), then verify the data
contract for shared vs. per-corner parameter independence:
- the shared `corner_r` binding survives the Unlink (other 3
corners still reference it).
- the per-corner `corner_r_ne` binding points at a fresh
parameter, distinct from `corner_r`.
- editing the shared `corner_r` rewrites its parameter; the
per-corner override stays at its original value (and vice
versa). This is the "pin one corner, edit the rest" workflow
parity Fusion ships.

NOTE: the Arc geometry doesn't currently re-read the parameter
table on solve (no constraint binds anchor / inset Points to the
shared / per-corner parameters; no post-solve mirror analogous
to `mirror_solve_to_chamfer_anchors` for RoundRect arcs). So this
test pins the **parameter-table** independence — the surface the
future Phase-6 constraint or mirror would read from. The Phase-6
follow-up will add direct geometry-radius assertions; **deferred
to Phase 6**, flagged in the report.
[test]

## Source
Lines 753–893 in `crates/oxide-app/tests/regression/library_cross_track.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_cross_track](/crates/oxide-app/tests/regression/library_cross_track.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
