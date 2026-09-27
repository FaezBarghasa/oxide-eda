---
okf_version: "0.2"
type: Function
title: trace_from_segment
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/trace_from_segment
language: rust
---

# trace_from_segment

## Signature

```rust
pub(super) fn trace_from_segment(segment: &Segment) -> TraceInput
```

## Visibility

- `pub(super)`

## Source
Lines 5–12 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [point_to_xy](/crates/oxide-renderer/src/pcb/emit/point_to_xy.md) |
| calls | [mm_with_floor](/crates/oxide-renderer/src/pcb/emit/mm_with_floor.md) |
