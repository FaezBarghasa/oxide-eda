---
okf_version: "0.2"
type: Function
title: build
description: Build spatial index from a PCB board definition.
resource: crates/oxide-router/src/geometry/rtree.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/geometry/rtree/build_1
language: rust
---

# build

Build spatial index from a PCB board definition.

## Signature

```rust
pub fn build(board: &PcbBoard) -> Self
```

## Visibility

- `pub`

## Docstring

Build spatial index from a PCB board definition.

## Source
Lines 53–124 in `crates/oxide-router/src/geometry/rtree.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rtree](/crates/oxide-router/src/geometry/rtree.md) |
