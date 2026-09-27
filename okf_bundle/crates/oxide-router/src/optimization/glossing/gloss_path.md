---
okf_version: "0.2"
type: Function
title: gloss_path
description: "Gloss a routing path: remove redundant corners and shorten track length."
resource: crates/oxide-router/src/optimization/glossing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:58:57Z"
concept_id: crates/oxide-router/src/optimization/glossing/gloss_path
language: rust
---

# gloss_path

Gloss a routing path: remove redundant corners and shorten track length.

## Signature

```rust
impl GlossingOptimizer { pub fn gloss_path(&self, path: &mut RoutingPath) -> GlossingResult }
```

## Visibility

- `pub`

## Docstring

Gloss a routing path: remove redundant corners and shorten track length.

## Source
Lines 28–46 in `crates/oxide-router/src/optimization/glossing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glossing](/crates/oxide-router/src/optimization/glossing.md) |
