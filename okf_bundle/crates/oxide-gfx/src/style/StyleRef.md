---
okf_version: "0.2"
type: Class
title: StyleRef
description: Compact style reference sent alongside primitive data.
resource: crates/oxide-gfx/src/style.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/style/StyleRef
language: rust
---

# StyleRef

Compact style reference sent alongside primitive data.

## Signature

```rust
pub struct StyleRef
```

## Decorators

- `repr(C)`
- `derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)`

## Visibility

- `pub`

## Docstring

Compact style reference sent alongside primitive data.
[repr(C)]
[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]

## Methods

- `slot`
- `flags`
- `alpha_mul`
- `_pad`

## Source
Lines 30–35 in `crates/oxide-gfx/src/style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [style](/crates/oxide-gfx/src/style.md) |
