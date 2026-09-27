---
okf_version: "0.2"
type: Function
title: text_position_px
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/text_position_px
language: rust
---

# text_position_px

## Signature

```rust
fn text_position_px(item: &TextItem, scale_px_per_mm: f32, offset_px: [f32; 2]) -> [f32; 2]
```

## Source
Lines 22–32 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| called_by | [glyphon_helpers_map_size_position_and_bounds](/crates/oxide-gfx/src/pipeline/text/glyphon_helpers_map_size_position_and_bounds.md) |
| called_by | [upload](/crates/oxide-gfx/src/pipeline/text/upload.md) |
