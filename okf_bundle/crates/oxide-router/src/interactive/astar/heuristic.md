---
okf_version: "0.2"
type: Function
title: heuristic
resource: crates/oxide-router/src/interactive/astar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T10:19:36Z"
concept_id: crates/oxide-router/src/interactive/astar/heuristic
language: rust
---

# heuristic

## Signature

```rust
fn heuristic(a: GridCoord, b: GridCoord) -> i64
```

## Source
Lines 244–250 in `crates/oxide-router/src/interactive/astar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [astar](/crates/oxide-router/src/interactive/astar.md) |
| called_by | [find_astar_path](/crates/oxide-router/src/interactive/astar/find_astar_path.md) |
| called_by | [find_astar_path_with_ml](/crates/oxide-router/src/interactive/astar/find_astar_path_with_ml.md) |
