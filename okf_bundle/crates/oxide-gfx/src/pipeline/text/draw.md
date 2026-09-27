---
okf_version: "0.2"
type: Function
title: draw
resource: crates/oxide-gfx/src/pipeline/text.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:47:43Z"
concept_id: crates/oxide-gfx/src/pipeline/text/draw
language: rust
---

# draw

## Signature

```rust
impl GlyphonTextPipeline { pub fn draw(
        &self,
        render_pass: &mut wgpu::RenderPass<'_>,
    ) -> Result<(), glyphon::RenderError> }
```

## Visibility

- `pub`

## Source
Lines 373–383 in `crates/oxide-gfx/src/pipeline/text.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [text](/crates/oxide-gfx/src/pipeline/text.md) |
