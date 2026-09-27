---
okf_version: "0.2"
type: Function
title: generate_footprint
description: Mathematically compiles package dimensions into an IPC-7351C compliant Footprint.
resource: crates/oxide-bake/src/ipc7351/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:10:48Z"
concept_id: crates/oxide-bake/src/ipc7351/mod/generate_footprint
language: rust
---

# generate_footprint

Mathematically compiles package dimensions into an IPC-7351C compliant Footprint.

## Signature

```rust
impl Ipc7351Generator { pub fn generate_footprint(name: &str, dims: &PackageDimensions, density: DensityLevel) -> Footprint }
```

## Visibility

- `pub`

## Docstring

Mathematically compiles package dimensions into an IPC-7351C compliant Footprint.

## Source
Lines 20–287 in `crates/oxide-bake/src/ipc7351/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ipc7351](/crates/oxide-bake/src/ipc7351/mod.md) |
| calls | [calculate_ipc7351c_pad](/crates/oxide-bake/src/ipc7351/calculator/calculate_ipc7351c_pad.md) |
| calls | [synthesize_thermal_paste_panes](/crates/oxide-bake/src/ipc7351/calculator/synthesize_thermal_paste_panes.md) |
