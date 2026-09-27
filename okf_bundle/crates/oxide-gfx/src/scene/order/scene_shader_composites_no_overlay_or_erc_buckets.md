---
okf_version: "0.2"
type: Function
title: scene_shader_composites_no_overlay_or_erc_buckets
description: Neither draw order composites overlay or ERC buckets through the
resource: crates/oxide-gfx/src/scene/order.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/order/scene_shader_composites_no_overlay_or_erc_buckets
language: rust
---

# scene_shader_composites_no_overlay_or_erc_buckets

Neither draw order composites overlay or ERC buckets through the

## Signature

```rust
fn scene_shader_composites_no_overlay_or_erc_buckets()
```

## Decorators

- `test`

## Docstring

Neither draw order composites overlay or ERC buckets through the
shader's base pass: overlay geometry gets its own dedicated pass
strictly after `GPU_SCENE_DRAW_ORDER` (see
`scene_shader::ScenePrimitive::draw`), and ERC markers are
schematic-only.
[test]

## Source
Lines 317–328 in `crates/oxide-gfx/src/scene/order.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [order](/crates/oxide-gfx/src/scene/order.md) |
