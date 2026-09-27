---
okf_version: "0.2"
type: Function
title: rectangle_sketch
description: "Build a sketch with one rectangle (4 Points + 4 Lines), solve,"
resource: crates/oxide-bake/src/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/src/profile/rectangle_sketch
language: rust
---

# rectangle_sketch

Build a sketch with one rectangle (4 Points + 4 Lines), solve,

## Signature

```rust
fn rectangle_sketch() -> (SketchData, SketchEntityId)
```

## Docstring

Build a sketch with one rectangle (4 Points + 4 Lines), solve,
trace from the first Line, expect a 4-vertex polygon.

## Source
Lines 389–436 in `crates/oxide-bake/src/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-bake/src/profile.md) |
| called_by | [trace_construction_lines_skipped](/crates/oxide-bake/src/profile/trace_construction_lines_skipped.md) |
| called_by | [trace_rectangle_closes](/crates/oxide-bake/src/profile/trace_rectangle_closes.md) |
