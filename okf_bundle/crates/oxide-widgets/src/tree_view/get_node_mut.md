---
okf_version: "0.2"
type: Function
title: get_node_mut
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/get_node_mut
language: rust
---

# get_node_mut

## Signature

```rust
fn get_node_mut(roots: &'a mut [TreeNode], path: &[usize]) -> Option<&'a mut TreeNode>
```

## Type Parameters

- `'a`

## Source
Lines 647–656 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
| called_by | [toggle](/crates/oxide-widgets/src/tree_view/toggle.md) |
