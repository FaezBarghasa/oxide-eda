---
okf_version: "0.2"
type: Class
title: ThermalParams
description: "[repr(C)]"
resource: crates/oxide-compute/src/thermal.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/thermal/ThermalParams
language: rust
---

# ThermalParams

[repr(C)]

## Signature

```rust
pub struct ThermalParams
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
- `ambient_temp`
- `dt`
- `copper_conductivity`
- `fr4_conductivity`
- `_pad0`
- `_pad1`

## Source
Lines 8–17 in `crates/oxide-compute/src/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/oxide-compute/src/thermal.md) |
