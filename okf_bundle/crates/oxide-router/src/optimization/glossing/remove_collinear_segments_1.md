---
okf_version: "0.2"
type: Function
title: remove_collinear_segments
resource: crates/oxide-router/src/optimization/glossing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/optimization/glossing/remove_collinear_segments_1
language: rust
---

# remove_collinear_segments

## Signature

```rust
fn remove_collinear_segments(&self, segments: &mut Vec<RouteSegment>) -> usize
```

## Source
Lines 48–73 in `crates/oxide-router/src/optimization/glossing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glossing](/crates/oxide-router/src/optimization/glossing.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
