---
okf_version: "0.2"
type: Class
title: GpuViolation
description: GPU representation of a clearance violation
resource: crates/oxide-compute/src/drc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/drc/GpuViolation
language: rust
---

# GpuViolation

GPU representation of a clearance violation

## Signature

```rust
pub struct GpuViolation
```

## Decorators

- `repr(C)`
- `derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

GPU representation of a clearance violation
[repr(C)]
[derive(Debug, Clone, Copy, Default, Pod, Zeroable, Serialize, Deserialize)]

## Methods

- `obj_a`
- `obj_b`
- `distance`
- `required`
- `rule_type`
- `x`
- `y`
- `_pad`

## Source
Lines 24–33 in `crates/oxide-compute/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-compute/src/drc.md) |
