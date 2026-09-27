---
okf_version: "0.2"
type: Function
title: upload
description: Upload line instances into the instance buffer.
resource: crates/oxide-gfx/src/pipeline/line.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/line/upload_1
language: rust
---

# upload

Upload line instances into the instance buffer.

## Signature

```rust
pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, lines: &[LineSegment])
```

## Visibility

- `pub`

## Docstring

Upload line instances into the instance buffer.

## Source
Lines 136–146 in `crates/oxide-gfx/src/pipeline/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/pipeline/line.md) |
