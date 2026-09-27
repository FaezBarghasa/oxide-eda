---
okf_version: "0.2"
type: Class
title: GraphicRotationPivotMode
description: Pivot mode for Symbol graphic rotation.
resource: crates/oxide-app/src/library/editor/symbol/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/mod/GraphicRotationPivotMode
language: rust
---

# GraphicRotationPivotMode

Pivot mode for Symbol graphic rotation.

## Signature

```rust
pub enum GraphicRotationPivotMode
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Pivot mode for Symbol graphic rotation.

`WorldOrigin` preserves legacy behavior where geometry orbits `(0, 0)`.
`GeometryCenter` rotates each graphic around its own center.
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 92–95 in `crates/oxide-app/src/library/editor/symbol/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/symbol/state/mod.md) |
