---
okf_version: "0.2"
type: Class
title: ArcScreenSpan
description: "How an [`oxide_gfx::primitive::arc::Arc`] maps onto the screen-space"
resource: crates/oxide-app/src/renderer_scene_canvas.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/renderer_scene_canvas/ArcScreenSpan
language: rust
---

# ArcScreenSpan

How an [`oxide_gfx::primitive::arc::Arc`] maps onto the screen-space

## Signature

```rust
pub enum ArcScreenSpan
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

How an [`oxide_gfx::primitive::arc::Arc`] maps onto the screen-space
angles `canvas::path::Arc` wants.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `start`
- `end`

## Source
Lines 52–60 in `crates/oxide-app/src/renderer_scene_canvas.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [renderer_scene_canvas](/crates/oxide-app/src/renderer_scene_canvas.md) |
