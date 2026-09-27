---
okf_version: "0.2"
type: Function
title: upload
resource: crates/oxide-gfx/src/pipeline/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/arc/upload
language: rust
---

# upload

## Signature

```rust
impl ArcPipeline { pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, arcs: &[Arc]) }
```

## Visibility

- `pub`

## Source
Lines 120–145 in `crates/oxide-gfx/src/pipeline/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/pipeline/arc.md) |
| calls | [ensure_capacity](/crates/oxide-gfx/src/pipeline/growth/ensure_capacity.md) |
