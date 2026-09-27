---
okf_version: "0.2"
type: Function
title: download_raw
resource: crates/oxide-compute/src/cpu_backend.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/cpu_backend/download_raw
language: rust
---

# download_raw

## Signature

```rust
impl CpuBackend { fn download_raw(
        &mut self,
        buffer_id: BufferId,
        out_bytes: &mut [u8],
    ) -> Result<(), ComputeError> }
```

## Source
Lines 67–79 in `crates/oxide-compute/src/cpu_backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cpu_backend](/crates/oxide-compute/src/cpu_backend.md) |
