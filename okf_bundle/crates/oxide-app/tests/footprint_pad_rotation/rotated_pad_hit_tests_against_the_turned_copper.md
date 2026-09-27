---
okf_version: "0.2"
type: Function
title: rotated_pad_hit_tests_against_the_turned_copper
description: A 2×1 mm pad turned 90° occupies ±0.5 mm in X and ±1.0 mm in Y.
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/rotated_pad_hit_tests_against_the_turned_copper
language: rust
---

# rotated_pad_hit_tests_against_the_turned_copper

A 2×1 mm pad turned 90° occupies ±0.5 mm in X and ±1.0 mm in Y.

## Signature

```rust
fn rotated_pad_hit_tests_against_the_turned_copper()
```

## Decorators

- `test`

## Docstring

A 2×1 mm pad turned 90° occupies ±0.5 mm in X and ±1.0 mm in Y.
The old `contains_mm` tested the un-rotated box, so it answered
exactly backwards on both probes.
[test]

## Source
Lines 74–88 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
