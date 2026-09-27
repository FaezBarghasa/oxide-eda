---
okf_version: "0.2"
type: Function
title: properties_panel_rotation_moves_the_sketch_outline_corners
description: "The Properties-panel rotation field is the third sibling: it also"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/properties_panel_rotation_moves_the_sketch_outline_corners
language: rust
---

# properties_panel_rotation_moves_the_sketch_outline_corners

The Properties-panel rotation field is the third sibling: it also

## Signature

```rust
fn properties_panel_rotation_moves_the_sketch_outline_corners()
```

## Decorators

- `test`

## Docstring

The Properties-panel rotation field is the third sibling: it also
writes `rotation_deg` and syncs without re-placing the corners.
[test]

## Source
Lines 393–410 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [sketched_pad_fixture](/crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture.md) |
| calls | [assert_corners_match_pad](/crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad.md) |
