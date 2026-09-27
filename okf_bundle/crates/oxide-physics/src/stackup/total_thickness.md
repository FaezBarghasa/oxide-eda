---
okf_version: "0.2"
type: Function
title: total_thickness
description: Calculate total thickness across all layers in micrometers.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/total_thickness
language: rust
---

# total_thickness

Calculate total thickness across all layers in micrometers.

## Signature

```rust
impl LayerStackup { pub fn total_thickness(&self) -> Microns }
```

## Visibility

- `pub`

## Docstring

Calculate total thickness across all layers in micrometers.

## Source
Lines 165–167 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
