---
okf_version: "0.2"
type: Function
title: anchored_top_left_px
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/anchored_top_left_px
language: rust
---

# anchored_top_left_px

## Signature

```rust
fn anchored_top_left_px(item: &TextItem, anchor_px: [f32; 2], size_px: [f32; 2]) -> [f32; 2]
```

## Source
Lines 73–81 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| calls | [alignment_offset_px](/crates/oxide-gfx/src/pipeline/text/alignment_offset_px.md) |
| calls | [rotated_offset_px](/crates/oxide-gfx/src/pipeline/text/rotated_offset_px.md) |
| called_by | [anchored_position_respects_alignment_and_rotation](/crates/oxide-gfx/src/pipeline/text/anchored_position_respects_alignment_and_rotation.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
