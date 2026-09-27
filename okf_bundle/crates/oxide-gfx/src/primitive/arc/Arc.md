---
okf_version: "0.2"
type: Class
title: Arc
description: Circular arc with start/end angles in radians.
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/Arc
language: rust
---

# Arc

Circular arc with start/end angles in radians.

## Signature

```rust
pub struct Arc
```

## Decorators

- `repr(C)`
- `derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)`

## Visibility

- `pub`

## Docstring

Circular arc with start/end angles in radians.
[repr(C)]
[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]

## Methods

- `center`
- `radius`
- `start_angle`
- `end_angle`
- `width`
- `color`
- `_pad`

## Source
Lines 10–18 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
