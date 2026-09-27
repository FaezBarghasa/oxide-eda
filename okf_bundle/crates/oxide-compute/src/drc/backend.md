---
okf_version: "0.2"
type: Function
title: backend
resource: crates/oxide-compute/src/drc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/drc/backend
language: rust
---

# backend

## Signature

```rust
impl GpuDrcChecker { pub fn backend(&self) -> &dyn ComputeBackend }
```

## Visibility

- `pub`

## Source
Lines 61–63 in `crates/oxide-compute/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-compute/src/drc.md) |
