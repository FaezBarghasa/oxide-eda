---
okf_version: "0.2"
type: Function
title: pan_moved_past_threshold_uses_cumulative_not_per_frame_delta
description: Many per-frame steps each under the 2px threshold still latch
resource: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/input/pointer/pan_moved_past_threshold_uses_cumulative_not_per_frame_delta
language: rust
---

# pan_moved_past_threshold_uses_cumulative_not_per_frame_delta

Many per-frame steps each under the 2px threshold still latch

## Signature

```rust
fn pan_moved_past_threshold_uses_cumulative_not_per_frame_delta()
```

## Decorators

- `test`

## Docstring

Many per-frame steps each under the 2px threshold still latch
once their CUMULATIVE displacement from the fixed origin
crosses it — a slow, deliberate drag, not 1px jitter.
[test]

## Source
Lines 428–439 in `crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pointer](/crates/oxide-app/src/library/editor/symbol/canvas/input/pointer.md) |
