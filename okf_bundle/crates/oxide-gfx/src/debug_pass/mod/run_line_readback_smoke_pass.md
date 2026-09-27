---
okf_version: "0.2"
type: Function
title: run_line_readback_smoke_pass
description: "Render one dashed [`LineSegment`] and read back the framebuffer's red"
resource: crates/oxide-gfx/src/debug_pass/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:35:56Z"
concept_id: crates/oxide-gfx/src/debug_pass/mod/run_line_readback_smoke_pass
language: rust
---

# run_line_readback_smoke_pass

Render one dashed [`LineSegment`] and read back the framebuffer's red

## Signature

```rust
fn run_line_readback_smoke_pass(
    line: LineSegment,
    min_stroke_px: f32,
    sample_x_px: &[u32],
) -> Result<Vec<u8>, String>
```

## Decorators

- `cfg(test)`

## Docstring

Render one dashed [`LineSegment`] and read back the framebuffer's red
channel at a handful of x pixels along the line's centre row. Proves
`line.wgsl` actually honours `LineSegment::STYLE_DASHED` on the GPU
(rather than just in the Rust-side `is_dashed()` predicate `order.rs`
pins): a solid white line over a black clear reads bright everywhere,
while a real dash pattern reads bright only where the shader's dash math
says it should. Returns the sampled red-channel bytes (0-255) at the given
world-mm x offsets along the line.

Geometry is chosen so the shader's dash math (`dash_mm = 8 * mm_per_px`,
`gap_mm = 5 * mm_per_px`) lands on exact pixel boundaries at
`scale_px_per_mm = 4.0` (`mm_per_px = 0.25`): an 8px dash then a 5px gap,
repeating every 13px — independent of zoom, since `dash_mm *
scale_px_per_mm` always collapses back to the literal `8`.

`cfg(test)` (unlike the other smoke-pass helpers above, which are `pub`
for `tests/regression_golden.rs` to call as a separate crate): this one
has no caller outside `debug_pass::tests`.
[cfg(test)]

## Source
Lines 203–337 in `crates/oxide-gfx/src/debug_pass/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [debug_pass](/crates/oxide-gfx/src/debug_pass/mod.md) |
| called_by | [a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it](/crates/oxide-gfx/src/debug_pass/tests/a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it.md) |
| called_by | [line_wgsl_actually_renders_a_dashed_pattern](/crates/oxide-gfx/src/debug_pass/tests/line_wgsl_actually_renders_a_dashed_pattern.md) |
