---
okf_version: "0.2"
type: Function
title: from_bytes
description: Load an ONNX model from raw byte slice and optimize graph for inference
resource: crates/oxide-ml/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-ml"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-18T23:47:27Z"
concept_id: crates/oxide-ml/src/engine/from_bytes
language: rust
---

# from_bytes

Load an ONNX model from raw byte slice and optimize graph for inference

## Signature

```rust
impl InferenceEngine { pub fn from_bytes(kind: ModelKind, model_bytes: &[u8]) -> Result<Self, MlError> }
```

## Visibility

- `pub`

## Docstring

Load an ONNX model from raw byte slice and optimize graph for inference

## Source
Lines 16–39 in `crates/oxide-ml/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-ml/src/engine.md) |
