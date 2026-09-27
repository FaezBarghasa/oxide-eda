---
okf_version: "0.2"
type: Function
title: placement_input_line_length_and_angle_commit_at_polar_offset
description: "v0.14-footprint #24 — with BOTH a typed length and a typed angle"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_line_length_and_angle_commit_at_polar_offset
language: rust
---

# placement_input_line_length_and_angle_commit_at_polar_offset

v0.14-footprint #24 — with BOTH a typed length and a typed angle

## Signature

```rust
fn placement_input_line_length_and_angle_commit_at_polar_offset()
```

## Decorators

- `test`

## Docstring

v0.14-footprint #24 — with BOTH a typed length and a typed angle
pinned (in either placement slot), the Line second click commits
the endpoint at `first + (len @ angle°)`, ignoring the cursor's own
azimuth and distance. 10 mm @ 90° from the origin lands the
endpoint at (0, 10) even though the cursor sits at (20, 0).
[test]

## Source
Lines 1003–1093 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
