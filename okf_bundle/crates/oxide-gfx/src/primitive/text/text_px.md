---
okf_version: "0.2"
type: Function
title: text_px
description: "Rendered em size in logical pixels for `size_mm` at `scale_px_per_mm`."
resource: crates/oxide-gfx/src/primitive/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/text/text_px
language: rust
---

# text_px

Rendered em size in logical pixels for `size_mm` at `scale_px_per_mm`.

## Signature

```rust
pub fn text_px(size_mm: f32, scale_px_per_mm: f32, policy: TextSizePolicy) -> f32
```

## Visibility

- `pub`

## Docstring

Rendered em size in logical pixels for `size_mm` at `scale_px_per_mm`.

The single sizing rule: convert world millimetres to em via [`MM_PER_EM`],
scale to pixels, then clamp to the surface's readability policy.

## Source
Lines 45–48 in `crates/oxide-gfx/src/primitive/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/primitive/text.md) |
| called_by | [text_size_px](/crates/oxide-gfx/src/pipeline/text/text_size_px.md) |
| called_by | [scaling_both_bounds_matches_scaling_the_clamped_result](/crates/oxide-gfx/src/primitive/text/scaling_both_bounds_matches_scaling_the_clamped_result.md) |
| called_by | [taking_size_mm_as_em_renders_short_by_the_ratio](/crates/oxide-gfx/src/primitive/text/taking_size_mm_as_em_renders_short_by_the_ratio.md) |
