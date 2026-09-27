---
okf_version: "0.2"
type: Function
title: upload
resource: crates/oxide-compute/src/backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/backend/upload_1
language: rust
---

# upload

## Signature

```rust
pub fn upload(
        &mut self,
        buffer: BufferId,
        data: &[T],
    ) -> Result<(), ComputeError>
```

## Type Parameters

- `T: bytemuck::Pod`

## Visibility

- `pub`

## Source
Lines 113–120 in `crates/oxide-compute/src/backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [backend](/crates/oxide-compute/src/backend.md) |
