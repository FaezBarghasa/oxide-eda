---
okf_version: "0.2"
type: Function
title: walk_one_ring
description: "Walk one output ring starting at `start` (an intersection on"
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/walk_one_ring
language: rust
---

# walk_one_ring

Walk one output ring starting at `start` (an intersection on

## Signature

```rust
fn walk_one_ring(
    subject: &mut [Vertex],
    clip: &mut [Vertex],
    start: usize,
    op: BoolOp,
) -> Vec<Point2>
```

## Docstring

Walk one output ring starting at `start` (an intersection on
the subject ring). Returns the ring's vertex positions in
output order. Marks every intersection visited along the way
(in BOTH rings).

## Source
Lines 239–349 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| calls | [subject_start_pos_or_default](/crates/oxide-sketch/src/geom/boolean_general/subject_start_pos_or_default.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
