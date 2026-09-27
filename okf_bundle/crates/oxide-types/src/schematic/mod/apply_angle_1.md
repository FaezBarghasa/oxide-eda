---
okf_version: "0.2"
type: Function
title: apply_angle
description: "Compose a child rotation (degrees, clockwise positive in"
resource: crates/oxide-types/src/schematic/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:15:42Z"
concept_id: crates/oxide-types/src/schematic/mod/apply_angle_1
language: rust
---

# apply_angle

Compose a child rotation (degrees, clockwise positive in

## Signature

```rust
pub fn apply_angle(&self, child_deg: f64) -> f64
```

## Decorators

- `must_use`

## Visibility

- `pub`

## Docstring

Compose a child rotation (degrees, clockwise positive in
schematic-screen space) with the parent's rotation + mirror so
the rendered angle ends up correct.
[must_use]

## Source
Lines 286–295 in `crates/oxide-types/src/schematic/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic](/crates/oxide-types/src/schematic/mod.md) |
