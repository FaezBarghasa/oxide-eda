---
okf_version: "0.2"
type: Function
title: body3d_default_is_a_sensible_block
description: "`Body3D::default()` should give us a sensible block: visible"
resource: crates/oxide-app/src/library/editor/footprint/body3d.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/body3d/body3d_default_is_a_sensible_block
language: rust
---

# body3d_default_is_a_sensible_block

`Body3D::default()` should give us a sensible block: visible

## Signature

```rust
fn body3d_default_is_a_sensible_block()
```

## Decorators

- `test`

## Docstring

`Body3D::default()` should give us a sensible block: visible
(non-zero alpha + non-zero height) and a defined extrude shape.
[test]

## Source
Lines 246–258 in `crates/oxide-app/src/library/editor/footprint/body3d.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [body3d](/crates/oxide-app/src/library/editor/footprint/body3d.md) |
