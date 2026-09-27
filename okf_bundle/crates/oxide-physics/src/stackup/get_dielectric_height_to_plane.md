---
okf_version: "0.2"
type: Function
title: get_dielectric_height_to_plane
description: Get total dielectric height between a signal layer and its reference plane.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/get_dielectric_height_to_plane
language: rust
---

# get_dielectric_height_to_plane

Get total dielectric height between a signal layer and its reference plane.

## Signature

```rust
impl LayerStackup { pub fn get_dielectric_height_to_plane(
        &self,
        signal_layer_idx: usize,
        plane_layer_idx: usize,
    ) -> Option<Microns> }
```

## Visibility

- `pub`

## Docstring

Get total dielectric height between a signal layer and its reference plane.

## Source
Lines 251–273 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
