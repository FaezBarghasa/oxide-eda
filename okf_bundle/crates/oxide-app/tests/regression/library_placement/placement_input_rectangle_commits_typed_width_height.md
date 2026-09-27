---
okf_version: "0.2"
type: Function
title: placement_input_rectangle_commits_typed_width_height
description: v0.14-footprint — Rectangle accepts typed width/height during
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_rectangle_commits_typed_width_height
language: rust
---

# placement_input_rectangle_commits_typed_width_height

v0.14-footprint — Rectangle accepts typed width/height during

## Signature

```rust
fn placement_input_rectangle_commits_typed_width_height()
```

## Decorators

- `test`

## Docstring

v0.14-footprint — Rectangle accepts typed width/height during
placement. With width 6 and height 4 pinned (one in each slot), the
second click commits a 6×4 box anchored at the first corner,
ignoring the cursor's 10×10 position.
[test]

## Source
Lines 1167–1241 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
