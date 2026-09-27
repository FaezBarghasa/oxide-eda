---
okf_version: "0.2"
type: Function
title: pipeline_impl_block
description: "The `impl shader::Pipeline for ScenePipeline` block alone."
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/pipeline_impl_block
language: rust
---

# pipeline_impl_block

The `impl shader::Pipeline for ScenePipeline` block alone.

## Signature

```rust
fn pipeline_impl_block() -> &'static str
```

## Docstring

The `impl shader::Pipeline for ScenePipeline` block alone.
The impl's own closing brace is the first `}` at column 0 after
the opener; every method inside it closes at an indent.

## Source
Lines 492–501 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover](/crates/oxide-app/src/scene_shader/the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover.md) |
