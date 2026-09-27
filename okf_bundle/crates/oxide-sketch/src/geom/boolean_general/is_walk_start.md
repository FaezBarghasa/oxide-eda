---
okf_version: "0.2"
type: Function
title: is_walk_start
description: "True when intersection vertex `idx` is unvisited and matches"
resource: crates/oxide-sketch/src/geom/boolean_general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean_general/is_walk_start
language: rust
---

# is_walk_start

True when intersection vertex `idx` is unvisited and matches

## Signature

```rust
fn is_walk_start(verts: &[Vertex], idx: usize, op: BoolOp) -> bool
```

## Docstring

True when intersection vertex `idx` is unvisited and matches
the operation's "start here" rule.

## Source
Lines 214–233 in `crates/oxide-sketch/src/geom/boolean_general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean_general](/crates/oxide-sketch/src/geom/boolean_general.md) |
| called_by | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
