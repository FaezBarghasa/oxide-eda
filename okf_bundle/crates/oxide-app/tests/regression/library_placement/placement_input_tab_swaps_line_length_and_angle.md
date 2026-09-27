---
okf_version: "0.2"
type: Function
title: placement_input_tab_swaps_line_length_and_angle
description: "v0.14-footprint #24 — Tab toggles the focused Line dimension field"
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/placement_input_tab_swaps_line_length_and_angle
language: rust
---

# placement_input_tab_swaps_line_length_and_angle

v0.14-footprint #24 — Tab toggles the focused Line dimension field

## Signature

```rust
fn placement_input_tab_swaps_line_length_and_angle()
```

## Decorators

- `test`

## Docstring

v0.14-footprint #24 — Tab toggles the focused Line dimension field
between length and angle, stashing the inactive field in
`placement_input_other`. Each field keeps its own typed digits
across the round-trip (length "10" survives length→angle→length).
[test]

## Source
Lines 887–995 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
