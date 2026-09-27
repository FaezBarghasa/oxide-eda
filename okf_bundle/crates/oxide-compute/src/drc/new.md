---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-compute/src/drc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/drc/new
language: rust
---

# new

## Signature

```rust
impl GpuDrcChecker { pub fn new(mut backend: Box<dyn ComputeBackend>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 50–59 in `crates/oxide-compute/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/oxide-compute/src/drc.md) |
| calls | [PipelineId](/crates/oxide-compute/src/backend/PipelineId.md) |
