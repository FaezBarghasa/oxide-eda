---
okf_version: "0.2"
type: Function
title: rect_from_top_left_size
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/rect_from_top_left_size
language: rust
---

# rect_from_top_left_size

## Signature

```rust
fn rect_from_top_left_size(top_left_px: [f32; 2], size_px: [f32; 2]) -> RectPx
```

## Source
Lines 91–98 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| called_by | [clipping_helper_rejects_outside_rectangles](/crates/oxide-gfx/src/pipeline/text/clipping_helper_rejects_outside_rectangles.md) |
| called_by | [overlap_ratio_detects_dense_overlap](/crates/oxide-gfx/src/pipeline/text/overlap_ratio_detects_dense_overlap.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
