---
okf_version: "0.2"
type: Function
title: the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover
description: "`PrepareError::AtlasFull` is the only thing `upload` can fail"
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover
language: rust
---

# the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover

`PrepareError::AtlasFull` is the only thing `upload` can fail

## Signature

```rust
fn the_pipeline_overrides_trim_so_a_full_glyph_atlas_can_recover()
```

## Decorators

- `test`

## Docstring

`PrepareError::AtlasFull` is the only thing `upload` can fail
with, and `prepare` discards it. That is a one-frame loss only
while something releases atlas pages between frames — the
trait's own `trim` is a no-op, so without this override the
atlas never drains and every label stays gone for the rest of
the session (#599).
[test]

## Source
Lines 510–520 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| calls | [pipeline_impl_block](/crates/oxide-app/src/scene_shader/pipeline_impl_block.md) |
