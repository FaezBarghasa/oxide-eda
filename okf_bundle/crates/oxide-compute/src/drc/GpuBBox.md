---
okf_version: "0.2"
type: Class
title: GpuBBox
description: GPU representation of a board bounding box
resource: crates/oxide-compute/src/drc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/drc/GpuBBox
language: rust
---

# GpuBBox

GPU representation of a board bounding box

## Signature

```rust
pub struct GpuBBox
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

GPU representation of a board bounding box
[repr(C)]
[derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)]

## Methods

- `min_x`
- `min_y`
- `max_x`
- `max_y`
- `layer`
- `net_id`
- `object_type`
- `object_id`

## Source
Lines 10–19 in `crates/oxide-compute/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-compute/src/drc.md) |
