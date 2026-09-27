---
okf_version: "0.2"
type: Function
title: line_wgsl_actually_renders_a_dashed_pattern
description: "Correctness — thread #5: `line.wgsl` must actually paint gaps for a"
resource: crates/oxide-gfx/src/debug_pass/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/debug_pass/tests/line_wgsl_actually_renders_a_dashed_pattern
language: rust
---

# line_wgsl_actually_renders_a_dashed_pattern

Correctness — thread #5: `line.wgsl` must actually paint gaps for a

## Signature

```rust
fn line_wgsl_actually_renders_a_dashed_pattern()
```

## Decorators

- `test`

## Docstring

Correctness — thread #5: `line.wgsl` must actually paint gaps for a
`STYLE_DASHED` segment, not just carry the unused attribute. Samples the
rendered red channel at pixel x offsets chosen to land mid-dash and
mid-gap under the shader's `dash_mm = 8 * mm_per_px` / `gap_mm = 5 *
mm_per_px` pattern (see `run_line_dash_readback_smoke_pass`). Before the
fix every sample reads bright (solid line); after it, gap samples read
dark (background shows through).
[test]

## Source
Lines 34–71 in `crates/oxide-gfx/src/debug_pass/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-gfx/src/debug_pass/tests.md) |
| calls | [run_line_readback_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_line_readback_smoke_pass.md) |
