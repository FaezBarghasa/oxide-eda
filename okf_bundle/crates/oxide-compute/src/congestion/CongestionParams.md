---
okf_version: "0.2"
type: Class
title: CongestionParams
description: "[repr(C)]"
resource: crates/oxide-compute/src/congestion.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/congestion/CongestionParams
language: rust
---

# CongestionParams

[repr(C)]

## Signature

```rust
pub struct CongestionParams
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, Default, Pod, Zeroable)`

## Visibility

- `pub`

## Docstring

[repr(C)]
[derive(Debug, Clone, Copy, Default, Pod, Zeroable)]

## Methods

- `grid_width`
- `grid_height`
- `num_tracks`
- `cell_size`

## Source
Lines 8–13 in `crates/oxide-compute/src/congestion.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [congestion](/crates/oxide-compute/src/congestion.md) |
