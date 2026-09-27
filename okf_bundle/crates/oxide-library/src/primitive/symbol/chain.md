---
okf_version: "0.2"
type: Module
title: chain
description: "Endpoint-chaining core — join loose `Line`/`Arc` segments (schematic"
resource: crates/oxide-library/src/primitive/symbol/chain.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/chain
language: rust
---

# chain

Endpoint-chaining core — join loose `Line`/`Arc` segments (schematic

## Docstring

Endpoint-chaining core — join loose `Line`/`Arc` segments (schematic
symbol body strokes) end-to-end into a single closed polygon contour.

This is the algorithmic core behind the future "join into polygon"
symbol-editor action: the caller supplies whatever `Line`/`Arc`
graphics the user has selected (in any order, any direction) and gets
back one ordered, CCW-wound vertex ring, or a precise diagnosis of why
the segments don't form one.

## Arc convention (load-bearing — read before touching this file)

[`ChainSegment::Arc`] mirrors [`super::SymbolGraphicKind::Arc`]'s
`{ center, radius, start_deg, end_deg }` shape, so it must match that
type's real interpretation exactly.

- The point at angle `a` (degrees) on the circle is
`center + radius * (cos(a), sin(a))` — standard math convention, no
axis flips. Authoritative reference:
`crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`'s
`graphic_contains_point` `Arc` arm and its `ArcStart`/`ArcEnd`
handle-position arms just below it.
- The arc always sweeps **counter-clockwise** (increasing angle) from
`start_deg` to `end_deg`, wrapping through 360° when
`start_deg > end_deg`. Corroborated by two independent runtime
consumers computing the identical wraparound: `hit_test.rs:131-135`
(`if s <= e { a >= s && a <= e } else { a >= s || a <= e }`) and
`crates/oxide-gfx/src/shader/arc.wgsl:52-54`'s `normalize_angle`
(`sweep = normalize_angle(end_angle - start_angle)`, `in_sweep = a
<= sweep`).

`start_deg == end_deg` (mod 360°) is a **degenerate zero-sweep**
input, not an implicit full circle — every runtime consumer treats it
as a zero-length point: `hit_test.rs`'s `s <= e` range collapses to
the single value `[s, s]`, and `arc.wgsl`'s `sweep` is `0.0` there too
(`in_sweep` only true exactly on the start angle). This module rejects
such a segment up front — along with any other segment whose own
tessellated *length* is shorter than [`CHAIN_ENDPOINT_EPSILON_MM`] —
as [`ChainError::DegenerateSegment`], rather than silently
materialising a circle the user never actually drew.

A near-360° (but not exactly zero-sweep) arc is real, hit-test-
selectable geometry, not degenerate — it's gated on total
tessellated length, not the chord between its two (possibly very
close together) endpoints. See [`reject_sub_epsilon_segments`]'s doc
comment for why the distinction matters.

Known divergence, not this module's concern: the CPU canvas fallback
(`renderer_scene_canvas.rs`'s `draw_arc_bucket`) feeds
`start_angle`/`end_angle` straight into `iced`'s `canvas::path::Arc`
builder as a signed sweep, without the `normalize_angle` wraparound
the GPU shader and `hit_test.rs` both apply — being fixed separately.
This module follows the `hit_test.rs`/GPU-shader convention
(CCW-always, wraparound-normalised), which is the one every other
reader of `SymbolGraphicKind::Arc` should also treat as authoritative.

`crates/oxide-bake/src/profile.rs`'s `push_arc_interior_if_arc` uses
a *different* arc representation (`center`/`start`/`end` points plus
an explicit `sweep_ccw` flag) for PCB sketch profiles — it is not the
convention this module follows; `SymbolGraphicKind::Arc` carries no
such flag, so CCW-always (per `hit_test.rs`) is the only convention
consistent with the existing symbol data model.

## Relationships

| Type | Target |
|------|--------|
| related | [ChainSegment](/crates/oxide-library/src/primitive/symbol/chain/ChainSegment.md) |
| related | [ChainError](/crates/oxide-library/src/primitive/symbol/chain/ChainError.md) |
| related | [chain_into_closed_contour](/crates/oxide-library/src/primitive/symbol/chain/chain_into_closed_contour.md) |
| related | [validate_finite](/crates/oxide-library/src/primitive/symbol/chain/validate_finite.md) |
| related | [reject_sub_epsilon_segments](/crates/oxide-library/src/primitive/symbol/chain/reject_sub_epsilon_segments.md) |
| related | [polyline_length](/crates/oxide-library/src/primitive/symbol/chain/polyline_length.md) |
| related | [EndpointClusters](/crates/oxide-library/src/primitive/symbol/chain/EndpointClusters.md) |
| related | [build_endpoint_clusters](/crates/oxide-library/src/primitive/symbol/chain/build_endpoint_clusters.md) |
| related | [validate_topology](/crates/oxide-library/src/primitive/symbol/chain/validate_topology.md) |
| related | [walk_cycle](/crates/oxide-library/src/primitive/symbol/chain/walk_cycle.md) |
| related | [finalize_ring](/crates/oxide-library/src/primitive/symbol/chain/finalize_ring.md) |
| related | [is_collinear](/crates/oxide-library/src/primitive/symbol/chain/is_collinear.md) |
| related | [tessellate_segment](/crates/oxide-library/src/primitive/symbol/chain/tessellate_segment.md) |
| related | [point_at_deg](/crates/oxide-library/src/primitive/symbol/chain/point_at_deg.md) |
| related | [ref_index](/crates/oxide-library/src/primitive/symbol/chain/ref_index.md) |
| related | [dist_sq](/crates/oxide-library/src/primitive/symbol/chain/dist_sq.md) |
| related | [average_point](/crates/oxide-library/src/primitive/symbol/chain/average_point.md) |
| related | [signed_area_x2](/crates/oxide-library/src/primitive/symbol/chain/signed_area_x2.md) |
| related | [uf_find](/crates/oxide-library/src/primitive/symbol/chain/uf_find.md) |
| related | [uf_union](/crates/oxide-library/src/primitive/symbol/chain/uf_union.md) |
