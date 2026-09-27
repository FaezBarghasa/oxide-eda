---
okf_version: "0.2"
type: Function
title: upload
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/upload
language: rust
---

# upload

## Signature

```rust
impl GlyphonTextPipeline { pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        font_system: &mut glyphon::FontSystem,
        texts: &[TextItem],
        scale_px_per_mm: f32,
        size_policy: TextSizePolicy,
        family: &'static str,
        viewport_size_px: [u32; 2],
        offset_px: [f32; 2],
    ) -> Result<(), glyphon::PrepareError> }
```

## Visibility

- `pub`

## Source
Lines 267–371 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
| calls | [viewport_bounds](/crates/oxide-gfx/src/pipeline/text/viewport_bounds.md) |
| calls | [text_size_px](/crates/oxide-gfx/src/pipeline/text/text_size_px.md) |
| calls | [attrs_for_item](/crates/oxide-gfx/src/pipeline/text/attrs_for_item.md) |
| calls | [set_text](/crates/oxide-app/src/library/component_preview/updates/parameters/set_text.md) |
| calls | [text_position_px](/crates/oxide-gfx/src/pipeline/text/text_position_px.md) |
| calls | [measure_text_bounds_px](/crates/oxide-gfx/src/pipeline/text/measure_text_bounds_px.md) |
| calls | [anchored_top_left_px](/crates/oxide-gfx/src/pipeline/text/anchored_top_left_px.md) |
| calls | [rect_from_top_left_size](/crates/oxide-gfx/src/pipeline/text/rect_from_top_left_size.md) |
| calls | [rect_intersects_viewport](/crates/oxide-gfx/src/pipeline/text/rect_intersects_viewport.md) |
| calls | [to_glyphon_color](/crates/oxide-gfx/src/pipeline/text/to_glyphon_color.md) |
