---
okf_version: "0.2"
type: Function
title: upload_overlay
description: "Upload overlay circle instances into the dedicated overlay buffer,"
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/upload_overlay_1
language: rust
---

# upload_overlay

Upload overlay circle instances into the dedicated overlay buffer,

## Signature

```rust
pub fn upload_overlay(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        circles: &[Circle],
    )
```

## Visibility

- `pub`

## Docstring

Upload overlay circle instances into the dedicated overlay buffer,
drawn by [`Self::draw_overlay`] in a separate later pass.

## Source
Lines 141–156 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
