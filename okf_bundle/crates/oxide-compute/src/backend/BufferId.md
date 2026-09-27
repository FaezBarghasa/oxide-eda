---
okf_version: "0.2"
type: Class
title: BufferId
description: "[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]"
resource: crates/oxide-compute/src/backend.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-compute"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:06:40Z"
concept_id: crates/oxide-compute/src/backend/BufferId
language: rust
---

# BufferId

[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Signature

```rust
pub struct BufferId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

## Source
Lines 35–35 in `crates/oxide-compute/src/backend.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [backend](/crates/oxide-compute/src/backend.md) |
| called_by | [allocate_buffer_raw](/crates/oxide-compute/src/cpu_backend/allocate_buffer_raw.md) |
| called_by | [allocate_buffer_raw](/crates/oxide-compute/src/wgpu_backend/allocate_buffer_raw.md) |
