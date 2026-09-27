---
okf_version: "0.2"
type: Class
title: Scene
description: Flat primitive collection consumed by GPU pipelines.
resource: crates/oxide-gfx/src/scene/frame.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/scene/frame/Scene
language: rust
---

# Scene

Flat primitive collection consumed by GPU pipelines.

## Signature

```rust
pub struct Scene
```

## Decorators

- `derive(Default, Debug, Clone)`

## Visibility

- `pub`

## Docstring

Flat primitive collection consumed by GPU pipelines.
[derive(Default, Debug, Clone)]

## Methods

- `lines`
- `circles`
- `arcs`
- `polygons`
- `texts`
- `overlay_lines`
- `overlay_circles`
- `overlay_polygons`
- `erc_marker_lines`
- `erc_marker_circles`
- `erc_marker_polygons`
- `dirty`

## Source
Lines 17–30 in `crates/oxide-gfx/src/scene/frame.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [frame](/crates/oxide-gfx/src/scene/frame.md) |
