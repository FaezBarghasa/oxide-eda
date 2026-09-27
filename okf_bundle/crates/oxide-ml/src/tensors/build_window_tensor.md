---
okf_version: "0.2"
type: Function
title: build_window_tensor
description: "Construct a 4D tensor `[num_layers, grid_h, grid_w, num_features]` around `center`"
resource: crates/oxide-ml/src/tensors.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:16:47Z"
concept_id: crates/oxide-ml/src/tensors/build_window_tensor
language: rust
---

# build_window_tensor

Construct a 4D tensor `[num_layers, grid_h, grid_w, num_features]` around `center`

## Signature

```rust
impl BoardTensorBuilder { pub fn build_window_tensor(
        &self,
        center: Point2D,
        _current_layer: u32,
        target: Point2D,
        _net_id: u32,
        obstacles: &[(Point2D, u32)], // (location, layer)
    ) -> ArrayD<f32> }
```

## Visibility

- `pub`

## Docstring

Construct a 4D tensor `[num_layers, grid_h, grid_w, num_features]` around `center`

## Source
Lines 21–91 in `crates/oxide-ml/src/tensors.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tensors](/crates/oxide-ml/src/tensors.md) |
