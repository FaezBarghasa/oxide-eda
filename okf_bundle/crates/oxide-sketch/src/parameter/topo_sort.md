---
okf_version: "0.2"
type: Function
title: topo_sort
description: Iterative DFS topological sort. Returns parameter names in an
resource: crates/oxide-sketch/src/parameter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/parameter/topo_sort
language: rust
---

# topo_sort

Iterative DFS topological sort. Returns parameter names in an

## Signature

```rust
fn topo_sort(asts: &BTreeMap<String, ExprNode>) -> Result<Vec<String>, ExprError>
```

## Docstring

Iterative DFS topological sort. Returns parameter names in an
order where every dependency precedes its dependants. Errors with
[`ExprError::Cycle`] if a cycle is found.

## Source
Lines 125–180 in `crates/oxide-sketch/src/parameter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parameter](/crates/oxide-sketch/src/parameter.md) |
| calls | [collect_deps](/crates/oxide-sketch/src/parameter/collect_deps.md) |
| calls | [name_borrow](/crates/oxide-sketch/src/parameter/name_borrow.md) |
| called_by | [resolve](/crates/oxide-sketch/src/parameter/resolve.md) |
