---
okf_version: "0.2"
type: Function
title: touching_line_scores_the_rotated_pad_not_the_unrotated_box
description: Touching Line is a sibling of rubber-band select and scored pads
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/touching_line_scores_the_rotated_pad_not_the_unrotated_box
language: rust
---

# touching_line_scores_the_rotated_pad_not_the_unrotated_box

Touching Line is a sibling of rubber-band select and scored pads

## Signature

```rust
fn touching_line_scores_the_rotated_pad_not_the_unrotated_box()
```

## Decorators

- `test`

## Docstring

Touching Line is a sibling of rubber-band select and scored pads
against the same un-rotated box. A 2×1 mm pad turned 90° occupies
±0.5 mm in X and ±1.0 mm in Y, so the un-rotated box answers
backwards on both of these lines.
[test]

## Source
Lines 223–259 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [fixture](/crates/oxide-app/tests/footprint_pad_rotation/fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
