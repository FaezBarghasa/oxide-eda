---
okf_version: "0.2"
type: Function
title: a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it
description: "#645: the GPU had no screen-space minimum stroke width where both CPU"
resource: crates/oxide-gfx/src/debug_pass/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/debug_pass/tests/a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it
language: rust
---

# a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it

#645: the GPU had no screen-space minimum stroke width where both CPU

## Signature

```rust
fn a_stroke_floor_keeps_a_hairline_visible_when_the_cpu_would_clamp_it()
```

## Decorators

- `test`

## Docstring

#645: the GPU had no screen-space minimum stroke width where both CPU
replays clamp (0.6 px schematic, 0.5 px PCB), so a hairline faded toward
invisible as you zoomed out. The floor now rides in the camera uniform and
`line.wgsl` applies it as `max(width, min_stroke_px * mm_per_px)`.

Rendered, not reasoned about: a width the pass renders at ~0.08 px is
nearly black without the floor and solid with it. The two runs differ only
in `min_stroke_px`, so anything else that changed brightness would move
both.
[test]

## Source
Lines 83–119 in `crates/oxide-gfx/src/debug_pass/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-gfx/src/debug_pass/tests.md) |
| calls | [run_line_readback_smoke_pass](/crates/oxide-gfx/src/debug_pass/mod/run_line_readback_smoke_pass.md) |
