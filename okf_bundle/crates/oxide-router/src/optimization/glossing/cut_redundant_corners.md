---
okf_version: "0.2"
type: Function
title: cut_redundant_corners
resource: crates/oxide-router/src/optimization/glossing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/optimization/glossing/cut_redundant_corners
language: rust
---

# cut_redundant_corners

## Signature

```rust
impl GlossingOptimizer { fn cut_redundant_corners(&self, segments: &mut Vec<RouteSegment>) -> usize }
```

## Source
Lines 75–103 in `crates/oxide-router/src/optimization/glossing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glossing](/crates/oxide-router/src/optimization/glossing.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
