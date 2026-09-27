---
okf_version: "0.2"
type: Function
title: run
description: Run inference with input tensors
resource: crates/oxide-ml/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:47:27Z"
concept_id: crates/oxide-ml/src/engine/run_1
language: rust
---

# run

Run inference with input tensors

## Signature

```rust
pub fn run(&self, inputs: TVec<Tensor>) -> Result<TVec<Arc<Tensor>>, MlError>
```

## Visibility

- `pub`

## Docstring

Run inference with input tensors

## Source
Lines 46–54 in `crates/oxide-ml/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-ml/src/engine.md) |
