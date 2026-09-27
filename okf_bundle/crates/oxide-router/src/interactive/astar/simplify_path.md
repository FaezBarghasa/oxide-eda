---
okf_version: "0.2"
type: Function
title: simplify_path
resource: crates/oxide-router/src/interactive/astar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:19:36Z"
concept_id: crates/oxide-router/src/interactive/astar/simplify_path
language: rust
---

# simplify_path

## Signature

```rust
fn simplify_path(points: &[Point2D]) -> Vec<Point2D>
```

## Source
Lines 252–273 in `crates/oxide-router/src/interactive/astar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [astar](/crates/oxide-router/src/interactive/astar.md) |
| called_by | [find_astar_path](/crates/oxide-router/src/interactive/astar/find_astar_path.md) |
| called_by | [find_astar_path_with_ml](/crates/oxide-router/src/interactive/astar/find_astar_path_with_ml.md) |
