---
okf_version: "0.2"
type: Function
title: upload_overlay
description: "Upload overlay line instances into the dedicated overlay buffer, drawn"
resource: crates/oxide-gfx/src/pipeline/line.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/line/upload_overlay_1
language: rust
---

# upload_overlay

Upload overlay line instances into the dedicated overlay buffer, drawn

## Signature

```rust
pub fn upload_overlay(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        lines: &[LineSegment],
    )
```

## Visibility

- `pub`

## Docstring

Upload overlay line instances into the dedicated overlay buffer, drawn
by [`Self::draw_overlay`] in a separate later pass — see the struct
doc for why this is a second buffer rather than a shared one.

## Source
Lines 151–166 in `crates/oxide-gfx/src/pipeline/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/pipeline/line.md) |
