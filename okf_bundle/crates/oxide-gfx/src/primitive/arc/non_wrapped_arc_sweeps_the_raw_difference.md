---
okf_version: "0.2"
type: Function
title: non_wrapped_arc_sweeps_the_raw_difference
description: "A non-wrapped arc (`start <= end`, no seam crossing) sweeps"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/non_wrapped_arc_sweeps_the_raw_difference
language: rust
---

# non_wrapped_arc_sweeps_the_raw_difference

A non-wrapped arc (`start <= end`, no seam crossing) sweeps

## Signature

```rust
fn non_wrapped_arc_sweeps_the_raw_difference()
```

## Decorators

- `test`

## Docstring

A non-wrapped arc (`start <= end`, no seam crossing) sweeps
exactly the raw positive difference — the case that was
already correct before this fix, unchanged by it.
[test]

## Source
Lines 89–92 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
