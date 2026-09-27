---
okf_version: "0.2"
type: Function
title: equal_start_and_end_is_zero_sweep_not_a_full_circle
description: "A degenerate zero-sweep arc (start == end) stays a point, not"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/equal_start_and_end_is_zero_sweep_not_a_full_circle
language: rust
---

# equal_start_and_end_is_zero_sweep_not_a_full_circle

A degenerate zero-sweep arc (start == end) stays a point, not

## Signature

```rust
fn equal_start_and_end_is_zero_sweep_not_a_full_circle()
```

## Decorators

- `test`

## Docstring

A degenerate zero-sweep arc (start == end) stays a point, not
a full circle — matches `arc.wgsl`'s `normalize_angle(0) == 0`
and the zero-sweep-degenerate handling in `hit_test.rs`.
[test]

## Source
Lines 98–101 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
