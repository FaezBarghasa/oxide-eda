---
okf_version: "0.2"
type: Function
title: toggle
description: Toggle expand/collapse at the given path.
resource: crates/oxide-widgets/src/tree_view.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/tree_view/toggle
language: rust
---

# toggle

Toggle expand/collapse at the given path.

## Signature

```rust
pub fn toggle(roots: &mut [TreeNode], path: &[usize])
```

## Visibility

- `pub`

## Docstring

Toggle expand/collapse at the given path.

## Source
Lines 641–645 in `crates/oxide-widgets/src/tree_view.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tree_view](/crates/oxide-widgets/src/tree_view.md) |
| calls | [get_node_mut](/crates/oxide-widgets/src/tree_view/get_node_mut.md) |
