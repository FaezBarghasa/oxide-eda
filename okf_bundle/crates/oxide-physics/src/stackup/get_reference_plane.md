---
okf_version: "0.2"
type: Function
title: get_reference_plane
description: Find the nearest reference plane (GND or PWR plane) for a given signal layer index.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/get_reference_plane
language: rust
---

# get_reference_plane

Find the nearest reference plane (GND or PWR plane) for a given signal layer index.

## Signature

```rust
impl LayerStackup { pub fn get_reference_plane(&self, layer_idx: usize) -> Option<usize> }
```

## Visibility

- `pub`

## Docstring

Find the nearest reference plane (GND or PWR plane) for a given signal layer index.

Scans both upwards and downwards through dielectric layers and returns the index
of the closest `InternalPlane`.

## Source
Lines 204–248 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
