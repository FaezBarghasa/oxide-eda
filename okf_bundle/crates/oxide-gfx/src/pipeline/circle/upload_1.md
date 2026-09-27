---
okf_version: "0.2"
type: Function
title: upload
description: Upload circle instances into the instance buffer.
resource: crates/oxide-gfx/src/pipeline/circle.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:29:26Z"
concept_id: crates/oxide-gfx/src/pipeline/circle/upload_1
language: rust
---

# upload

Upload circle instances into the instance buffer.

## Signature

```rust
pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, circles: &[Circle])
```

## Visibility

- `pub`

## Docstring

Upload circle instances into the instance buffer.

## Source
Lines 127–137 in `crates/oxide-gfx/src/pipeline/circle.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [circle](/crates/oxide-gfx/src/pipeline/circle.md) |
