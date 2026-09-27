---
okf_version: "0.2"
type: Function
title: is_dashed
description: "Whether this segment renders dashed. The low `style` bit selects the"
resource: crates/oxide-gfx/src/primitive/line.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-gfx/src/primitive/line/is_dashed
language: rust
---

# is_dashed

Whether this segment renders dashed. The low `style` bit selects the

## Signature

```rust
impl LineSegment { pub fn is_dashed(&self) -> bool }
```

## Visibility

- `pub`

## Docstring

Whether this segment renders dashed. The low `style` bit selects the
dash pattern; the rest is reserved. This is the shared predicate the CPU
renderer honours and the GPU `line.wgsl` shader must match — the CPU↔GPU
parity test locks both paths to it.

## Source
Lines 27–29 in `crates/oxide-gfx/src/primitive/line.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [line](/crates/oxide-gfx/src/primitive/line.md) |
