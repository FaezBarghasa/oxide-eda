---
okf_version: "0.2"
type: Function
title: new
description: Create a circle pipeline bound to a target surface format.
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/new
language: rust
---

# new

Create a circle pipeline bound to a target surface format.

## Signature

```rust
impl CirclePipeline { pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a circle pipeline bound to a target surface format.

## Source
Lines 26–124 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
