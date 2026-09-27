---
okf_version: "0.2"
type: Function
title: map_corners
description: Translate sketch ChamferedCorners into the lib mirror enum.
resource: crates/oxide-bake/src/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-bake/src/pad/map_corners
language: rust
---

# map_corners

Translate sketch ChamferedCorners into the lib mirror enum.

## Signature

```rust
fn map_corners(c: &SkChamferedCorners) -> LibChamferedCorners
```

## Docstring

Translate sketch ChamferedCorners into the lib mirror enum.

## Source
Lines 343–350 in `crates/oxide-bake/src/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-bake/src/pad.md) |
| called_by | [bake_shape](/crates/oxide-bake/src/pad/bake_shape.md) |
