---
okf_version: "0.2"
type: Function
title: intersect_convex_clip
description: "Clip the (possibly concave) `subject` polygon against the"
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean/intersect_convex_clip
language: rust
---

# intersect_convex_clip

Clip the (possibly concave) `subject` polygon against the

## Signature

```rust
pub fn intersect_convex_clip(subject: &[Point2], clip: &[Point2]) -> Vec<Point2>
```

## Visibility

- `pub`

## Docstring

Clip the (possibly concave) `subject` polygon against the
**convex** `clip` polygon. Returns a single polygon ring
representing the intersection. An empty result means the
subject lies entirely outside the clip, or the inputs are
degenerate.

Algorithm: walk every edge of `clip`. For each edge, treat it
as a half-plane and pass the current subject through it,
keeping vertices on the inside and inserting intersection
points where edges cross the half-plane. The output of one
iteration becomes the input of the next.

The convex constraint on `clip` is what makes this work
without branching into separate output rings — any subject
edge can cross a clip edge at most twice (once entering,
once exiting), and the kept-inside vertices stay connected.

## Source
Lines 34–59 in `crates/oxide-sketch/src/geom/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-sketch/src/geom/boolean.md) |
| calls | [signed_area](/crates/oxide-sketch/src/geom/predicates/signed_area.md) |
| calls | [clip_against_edge](/crates/oxide-sketch/src/geom/boolean/clip_against_edge.md) |
| called_by | [concave_subject_against_convex_clip](/crates/oxide-sketch/src/geom/boolean/concave_subject_against_convex_clip.md) |
| called_by | [cw_clip_winding_normalised](/crates/oxide-sketch/src/geom/boolean/cw_clip_winding_normalised.md) |
| called_by | [full_overlap_returns_subject](/crates/oxide-sketch/src/geom/boolean/full_overlap_returns_subject.md) |
| called_by | [no_overlap_returns_empty](/crates/oxide-sketch/src/geom/boolean/no_overlap_returns_empty.md) |
| called_by | [partial_overlap_quarter_square](/crates/oxide-sketch/src/geom/boolean/partial_overlap_quarter_square.md) |
| called_by | [point_on_clip_edge_kept](/crates/oxide-sketch/src/geom/boolean/point_on_clip_edge_kept.md) |
