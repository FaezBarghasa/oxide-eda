---
okf_version: "0.2"
type: Function
title: rotate_local
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/rotate_local
language: rust
---

# rotate_local

## Signature

```rust
pub(super) fn rotate_local(point: [f32; 2], rotation_deg: f32) -> [f32; 2]
```

## Visibility

- `pub(super)`

## Source
Lines 120–129 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| called_by | [pad_from_footprint](/crates/oxide-renderer/src/pcb/emit/pad_from_footprint.md) |
