---
okf_version: "0.2"
type: Function
title: get_effective_er_between
description: Get the effective relative permittivity (Dk) of the dielectric between two layers.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/get_effective_er_between_1
language: rust
---

# get_effective_er_between

Get the effective relative permittivity (Dk) of the dielectric between two layers.

## Signature

```rust
pub fn get_effective_er_between(&self, layer_a_idx: usize, layer_b_idx: usize) -> Option<f64>
```

## Visibility

- `pub`

## Docstring

Get the effective relative permittivity (Dk) of the dielectric between two layers.

## Source
Lines 276–308 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
