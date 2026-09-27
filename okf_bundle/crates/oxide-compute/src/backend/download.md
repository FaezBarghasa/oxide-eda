---
okf_version: "0.2"
type: Function
title: download
resource: crates/oxide-compute/src/backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/backend/download
language: rust
---

# download

## Signature

```rust
impl dyn ComputeBackend { pub fn download(
        &mut self,
        buffer: BufferId,
        count: usize,
    ) -> Result<Vec<T>, ComputeError> }
```

## Type Parameters

- `T: bytemuck::Pod`

## Visibility

- `pub`

## Source
Lines 122–131 in `crates/oxide-compute/src/backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [backend](/crates/oxide-compute/src/backend.md) |
