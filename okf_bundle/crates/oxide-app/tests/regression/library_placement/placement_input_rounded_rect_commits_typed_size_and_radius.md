---
okf_version: "0.2"
type: Function
title: placement_input_rounded_rect_commits_typed_size_and_radius
description: "v0.14-footprint — Rounded-Rectangle commit honours typed width,"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_rounded_rect_commits_typed_size_and_radius
language: rust
---

# placement_input_rounded_rect_commits_typed_size_and_radius

v0.14-footprint — Rounded-Rectangle commit honours typed width,

## Signature

```rust
fn placement_input_rounded_rect_commits_typed_size_and_radius()
```

## Decorators

- `test`

## Docstring

v0.14-footprint — Rounded-Rectangle commit honours typed width,
height AND corner radius. Width 8 / height 5 / radius 1.5 pinned
across the slots; the committed box spans 8×5 and each corner arc
has radius 1.5, regardless of the cursor's position.
[test]

## Source
Lines 1342–1449 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
