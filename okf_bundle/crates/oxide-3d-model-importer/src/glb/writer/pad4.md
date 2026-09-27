---
okf_version: "0.2"
type: Function
title: pad4
description: "Pad a byte slice to the next 4-byte boundary using `fill`."
resource: crates/oxide-3d-model-importer/src/glb/writer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-3d-model-importer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-3d-model-importer/src/glb/writer/pad4
language: rust
---

# pad4

Pad a byte slice to the next 4-byte boundary using `fill`.

## Signature

```rust
fn pad4(data: &[u8], fill: T) -> Vec<u8>
```

## Type Parameters

- `T: Copy + Into<u8`

## Docstring

Pad a byte slice to the next 4-byte boundary using `fill`.

## Source
Lines 48–55 in `crates/oxide-3d-model-importer/src/glb/writer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [writer](/crates/oxide-3d-model-importer/src/glb/writer.md) |
| called_by | [write_glb](/crates/oxide-3d-model-importer/src/glb/writer/write_glb.md) |
