---
okf_version: "0.2"
type: Function
title: allocate_buffer
resource: crates/oxide-compute/src/backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/backend/allocate_buffer_1
language: rust
---

# allocate_buffer

## Signature

```rust
pub fn allocate_buffer(
        &mut self,
        data: &[T],
    ) -> Result<BufferId, ComputeError>
```

## Type Parameters

- `T: bytemuck::Pod`

## Visibility

- `pub`

## Source
Lines 97–103 in `crates/oxide-compute/src/backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [backend](/crates/oxide-compute/src/backend.md) |
