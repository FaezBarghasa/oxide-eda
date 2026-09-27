---
okf_version: "0.2"
type: Function
title: remove_loops
description: Automatically find and remove self-intersecting loops in a routing path.
resource: crates/oxide-router/src/optimization/loop_removal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:40Z"
concept_id: crates/oxide-router/src/optimization/loop_removal/remove_loops
language: rust
---

# remove_loops

Automatically find and remove self-intersecting loops in a routing path.

## Signature

```rust
impl LoopRemovalOptimizer { pub fn remove_loops(&self, path: &mut RoutingPath) -> LoopRemovalResult }
```

## Visibility

- `pub`

## Docstring

Automatically find and remove self-intersecting loops in a routing path.

## Source
Lines 27–57 in `crates/oxide-router/src/optimization/loop_removal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [loop_removal](/crates/oxide-router/src/optimization/loop_removal.md) |
