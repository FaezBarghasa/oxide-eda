---
okf_version: "0.2"
type: Class
title: LineSegment
description: Straight line segment with width and style.
resource: crates/oxide-gfx/src/primitive/line.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/primitive/line/LineSegment
language: rust
---

# LineSegment

Straight line segment with width and style.

## Signature

```rust
pub struct LineSegment
```

## Decorators

- `repr(C)`
- `derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)`

## Visibility

- `pub`

## Docstring

Straight line segment with width and style.
[repr(C)]
[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]

## Methods

- `p0`
- `p1`
- `width`
- `color`
- `style`
- `_pad`

## Source
Lines 10–17 in `crates/oxide-gfx/src/primitive/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/primitive/line.md) |
